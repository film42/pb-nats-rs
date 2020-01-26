#[macro_use]
extern crate failure;
extern crate pb_nats_rs;

use pb_nats_rs::pb::rpc::Request as RpcRequest;
use pb_nats_rs::pb::warehouse::{Shipment, ShipmentRequest, Shipments};
use pb_nats_rs::rpc::{build_request, GetResponseProtoable, Requestable, RpcClient};

use protobuf::Message;
use rants::Client;
use tower::Service;

pub enum ShipmentEndpoint {
    Search(ShipmentRequest),
    Update(Shipment),
    Create(Shipment),
    NotImplemented(Shipment),
}

impl Requestable for ShipmentEndpoint {
    fn service_name(&self) -> String {
        "Warehouse::ShipmentService".to_string()
    }
    fn method_name(&self) -> String {
        match self {
            ShipmentEndpoint::Search(_) => "search".to_string(),
            ShipmentEndpoint::Update(_) => "update".to_string(),
            ShipmentEndpoint::Create(_) => "create".to_string(),
            ShipmentEndpoint::NotImplemented(_) => "not_implemented".to_string(),
        }
    }
    fn into_bytes(&self) -> Result<Vec<u8>, failure::Error> {
        let bytes = match self {
            ShipmentEndpoint::Search(pb) => pb.write_to_bytes()?,
            ShipmentEndpoint::Update(pb) => pb.write_to_bytes()?,
            ShipmentEndpoint::Create(pb) => pb.write_to_bytes()?,
            ShipmentEndpoint::NotImplemented(pb) => pb.write_to_bytes()?,
        };
        Ok(bytes)
    }
}

pub struct ShipmentClient<S> {
    rpc_client: S,
}

impl<S> ShipmentClient<S>
where
    S: Service<RpcRequest> + Clone,
    <S as Service<RpcRequest>>::Response: GetResponseProtoable,
    <S as Service<RpcRequest>>::Error: std::fmt::Debug,
{
    async fn do_request<Req, Res>(&self, req: Req) -> Result<Res, failure::Error>
    where
        Req: Requestable,
        Res: protobuf::Message,
    {
        let mut rpc_client = self.rpc_client.clone();
        let req: RpcRequest = build_request(req)?;
        let res = rpc_client
            .call(req)
            .await
            .map_err(|e| format_err!("Error: {:?}", e))?;
        let mut res_pb = Res::new();
        res_pb.merge_from_bytes(res.get_response_proto())?;
        Ok(res_pb)
    }

    pub async fn search(&self, req: ShipmentRequest) -> Result<Shipments, failure::Error> {
        self.do_request(ShipmentEndpoint::Search(req)).await
    }

    pub async fn create(&self, req: Shipment) -> Result<Shipment, failure::Error> {
        self.do_request(ShipmentEndpoint::Create(req)).await
    }

    pub async fn not_implemented(&self, req: Shipment) -> Result<Shipment, failure::Error> {
        self.do_request(ShipmentEndpoint::NotImplemented(req)).await
    }

    pub async fn update(&self, req: Shipment) -> Result<Shipment, failure::Error> {
        self.do_request(ShipmentEndpoint::Update(req)).await
    }
}

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:4222".parse().unwrap();
    let client = Client::new(vec![address]);
    // Connect to the server
    client.connect().await;

    {
        use tower::ServiceBuilder;
        let rpc_client_srv = RpcClient::new(client.clone());
        let rpc_client = ServiceBuilder::new()
            .buffer(100)
            .concurrency_limit(100)
            .service(rpc_client_srv.clone());

        println!("Started!");
        let sc = ShipmentClient {
            rpc_client: rpc_client,
        };
        let req = ShipmentRequest::new();
        //let mut res: Shipments = sc.do_request(ShipmentEndpoint::Search(req)).await.unwrap();
        let mut res: Shipments = sc.search(req).await.unwrap();
        let mut shipment: Shipment = res.take_records().pop().unwrap();
        println!("First: {:?}", shipment);
        shipment.set_address("1337 Apple Ln".to_string());
        let res: Shipment = sc.create(shipment).await.unwrap();
        println!("Second: {:?}", res);
    }

    client.disconnect().await;
    println!("Done!");
}
