extern crate rants;
extern crate uuid;
#[macro_use]
extern crate failure;
// extern crate protoc_rust;
// use protoc_rust::Customize;

// mod atlas;
mod pb;
mod rpc;

use rpc::GetResponseProtoable;

use pb::warehouse::{Shipment, ShipmentRequest, Shipments};
use protobuf::Message;
use rants::Client;
use tower::Service;

#[derive(Clone)]
pub struct ShipmentsRpcClient {
    nats: Client,
}

//impl ShipmentsRpcClient {
//    fn new(nats: Client) -> ShipmentsRpcClient {
//        ShipmentsRpcClient { nats: nats }
//    }
//}

pub enum ShipmentEndpoint {
    Search(ShipmentRequest),
    Update(Shipment),
    Create(Shipment),
    NotImplemented(Shipment),
}

// impl rpc::Requestable for ShipmentRequest {
//     fn service_name(&self) -> String {
//         "Warehouse::ShipmentService".to_string()
//     }
//     fn method_name(&self) -> String {
//         "search".to_string()
//     }
//     fn into_bytes(&self) -> Result<Vec<u8>, failure::Error> {
//         let bytes = self.write_to_bytes()?;
//         Ok(bytes)
//     }
// }

// impl rpc::Requestable for Shipment {
//     fn service_name(&self) -> String {
//         "Warehouse::ShipmentService".to_string()
//     }
//     fn method_name(&self) -> String {
//         "create".to_string()
//     }
//     fn into_bytes(&self) -> Result<Vec<u8>, failure::Error> {
//         let bytes = self.write_to_bytes()?;
//         Ok(bytes)
//     }
// }

impl rpc::Requestable for ShipmentEndpoint {
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

// impl Service<ShipmentEndpoint> for ShipmentsRpcClient {
//     type Response = Shipments;
//     type Error = failure::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
//
//     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }
//
//     fn call(&mut self, req: ShipmentEndpoint) -> Self::Future {
//         let nats = self.nats.clone();
//         Box::pin(async move {
//             let res = rpc::call(&nats, req).await?;
//             let mut shipments = Shipments::new();
//             shipments.merge_from_bytes(res.get_response_proto())?;
//             Ok(shipments)
//         })
//     }
// }

use pb::rpc::Request as RpcRequest;

pub struct ShipmentClient<S> {
    rpc_client: S,
}

impl<S> ShipmentClient<S>
where
    S: Service<RpcRequest> + Clone,
    <S as Service<RpcRequest>>::Response: GetResponseProtoable,
    <S as Service<RpcRequest>>::Error: std::fmt::Debug,
    //<S as Service<RpcRequest>>::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    async fn do_request<Req, Res>(&self, req: Req) -> Result<Res, failure::Error>
    where
        Req: rpc::Requestable,
        Res: protobuf::Message,
    {
        let mut rpc_client = self.rpc_client.clone();
        let req: RpcRequest = rpc::build_request(req)?;
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
        let rpc_client_srv = rpc::RpcClient::new(client.clone());
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

    // {
    //     let sc = ShipmentClient{nats: client.clone()};
    //     let shipments = sc.search(ShipmentRequest::new()).await.unwrap();
    //     println!("Shipments: {:?}", shipments);
    // }

    // {
    //     use tower::{ServiceBuilder, ServiceExt};
    //     let mut rpc_client = ServiceBuilder::new()
    //         .buffer(100)
    //         .concurrency_limit(100)
    //         .service(ShipmentsRpcClient::new(client.clone()));

    //     let mut req = ShipmentRequest::new();
    //     let mut args = protobuf::RepeatedField::new();
    //     args.push("1337".to_string());
    //     req.set_guid(args);
    //     rpc_client.ready().await.unwrap();
    //     let mut res: Shipments = rpc_client
    //         .call(ShipmentEndpoint::Search(req))
    //         .await
    //         .unwrap();

    //     let mut shipment: Shipment = res.take_records().pop().unwrap();
    //     println!("First: {:?}", shipment);
    //     shipment.set_address("1337 Apple Ln".to_string());
    //     let res = rpc_client.call(ShipmentEndpoint::Create(shipment)).await;
    //     //.unwrap();
    //     println!("Second: {:?}", res);
    // }

    // let futures: Vec<_> = (1..100)
    //     .into_iter()
    //     .map(|i| {
    //         let mut rpc_client = rpc_client.clone();
    //         tokio::spawn(async move {
    //             let mut req = ShipmentRequest::new();
    //             let mut args = protobuf::RepeatedField::new();
    //             args.push(i.to_string());
    //             req.set_guid(args);
    //             match rpc_client.ready().await {
    //                 Ok(_) => {
    //                     println!("Got response for: {}", i);
    //                     match rpc_client.call(ShipmentEndpoint::Search(req)).await {
    //                         Ok(shipments) => {
    //                             //let shipments = search_shipments(&client).await.unwrap();
    //                             for shipment in shipments.get_records().iter() {
    //                                 println!("Shipment guid: {}", shipment.get_guid());
    //                             }
    //                         }
    //                         Err(err) => println!("Error: {}", err),
    //                     }
    //                 }
    //                 Err(err) => println!("Error: {}", err),
    //             };
    //         })
    //     })
    //     .collect();

    // futures.iter().map(|f| f.await);

    // futures::future::join_all(futures).await;

    // for i in 1..100 {
    //     let mut rpc_client = rpc_client.clone();

    //     //tokio::spawn(async move {
    //         let mut req = ShipmentRequest::new();
    //         let mut args = protobuf::RepeatedField::new();
    //         args.push(i.to_string());
    //         req.set_guid(args);

    //         match rpc_client.ready().await {
    //             Ok(_) => {
    //                 match rpc_client.call(req).await {
    //                     Ok(shipments) => {
    //                         //let shipments = search_shipments(&client).await.unwrap();
    //                         for shipment in shipments.get_records().iter() {
    //                             println!("Shipment guid: {}", shipment.get_guid());
    //                         }
    //                     },
    //                     Err(err) => {
    //                         println!("Error: {}", err);
    //                     }
    //                 };
    //             },
    //             Err(err) => {
    //                 println!("Error: {}", err);
    //             }
    //         }
    //     //});

    // }

    // tokio::time::delay_until(Instant::now() + Duration::from_secs(5)).await;

    client.disconnect().await;
    println!("Done!");
}
