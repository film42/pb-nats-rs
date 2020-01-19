use super::pb::rpc::{Request as RpcRequest, Response as RpcResponse};
use super::pb::warehouse::*;
use futures::stream::StreamExt;
use protobuf::Message;
use rants::{Client, Subject};
use tokio::timer::Timeout;
use uuid::Uuid;
use std::future::Future;
use std::task::{Context,Poll};
use std::pin::Pin;

pub struct RpcClient {
    nats: Client,
}

fn new_inbox() -> Subject {
    Uuid::new_v4().to_string().parse().unwrap()
}

fn to_snake_case(s: String) -> String {
    let mut out = vec![];
    let mut is_first = true;
    let mut last_was_alphabetic = false;
    for c in s.chars() {
        if c.is_uppercase() {
            if !is_first && last_was_alphabetic {
                out.push('_');
            }
            for c0 in c.to_lowercase().to_string().chars() {
                out.push(c0);
            }
        } else {
            out.push(c);
        }
        is_first = false;
        last_was_alphabetic = c.is_ascii_alphabetic();
    }
    out.iter().collect()
}

// example: rpc.warehouse.shipment_service.search
pub fn build_subject_from_request(req: &RpcRequest) -> String {
    let service = req.get_service_name();
    let service = service.replace("::", ".");
    let service = to_snake_case(service);
    let method = req.get_method_name().to_string();
    let method = to_snake_case(method);
    format!("rpc.{}.{}", service, method)
}

impl RpcClient {
    async fn call(&self, request: &mut RpcRequest) -> Result<RpcResponse, failure::Error> {
        request.set_caller("pb-nats-rs".to_string());

        let subject = build_subject_from_request(request).parse()?;
        let inbox = new_inbox();

        let (sid, mut sub) = self.nats.subscribe(&inbox, 2).await?;
        self.nats.unsubscribe_with_max_msgs(sid, 2).await?;

        let payload = request.write_to_bytes().unwrap();
        self.nats
            .publish_with_reply(&subject, &inbox, &payload)
            .await?;

        let msg1 = Timeout::new(sub.next(), super::ACK_TIMEOUT).await?.unwrap();

        match msg1.payload() {
            &[super::NACK_MSG] => bail!("Retry later! Server sent NACK"),
            &[super::ACK_MSG] => {}
            _ => bail!("Received unknown message!"),
        }

        // if msg1.payload() != ACK_MSG.as_bytes() {
        //     // TODO: Make this a lot better.
        //     bail!("Did not get an ACK. Why?");
        // }

        let msg2 = Timeout::new(sub.next(), super::RESPONSE_TIMEOUT)
            .await?
            .unwrap();

        // let res = std::str::from_utf8(&msg2.payload())?.to_string();
        // println!("The response is: {}", res);

        let mut res = RpcResponse::new();
        res.merge_from_bytes(&msg2.payload())?;

        Ok(res)
    }
}

pub struct ShipmentClient {
    client: RpcClient,
}

pub struct SearchRequest<Req, Res> {
    req: Req,
    client: RpcClient,
    future: String,
}

impl <Req>SearchRequest {
    fn new(client: RpcClient, req: Req) -> SearchRequest<Req> {
        SearchRequest{
            req: req,
            client: client,
            future: "".into(),
        }
    }
}

impl <Res>Future for SearchRequest<Res>
    where Res: std::marker::Unpin
{
    type Output = Result<Res, failure::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut me = Pin::new(&mut *self);
        Poll::Pending
    }
}

pub trait Search {
    type Request;
    type Response;
    fn search(&self, request: Self::Request) -> SearchRequest<Self::Response>;
}

impl ShipmentClient {
    fn new(client: RpcClient) -> ShipmentClient {
        ShipmentClient { client: client }
    }
}

// impl Search for ShipmentClient {
//     type Request = ShipmentRequest;
//     type Response = Shipments;

//     fn search(&self, request: Self::Request) -> Result<Self::Response, failure::Error> {
//         let req_bytes = request.write_to_bytes()?;

//         let mut pb_req = RpcRequest::new();
//         pb_req.set_request_proto(req_bytes);
//         pb_req.set_service_name("Warehouse::ShipmentService".to_string());
//         pb_req.set_method_name("search".to_string());

//         let res = self.client.call(&pb_req).await?;

//         let mut shipments = Self::Response::new();
//         shipments.merge_from_bytes(res.get_response_proto())?;
//         //println!("Msg: {}", shipments.get_records()[0].get_guid());
//         Ok(shipments)
//     }
// }

// fn asdf() {
//     let cs = ClientService{
//         service_name: "Warehouse::ShipmentService".to_string(),
//     }
//     cs.register(MethodHandler{
//         method_name: "search".to_string(),
//     });

//     let sd = ServiceDescription{
//         service_name: "astlas.dispatch.ShardService".to_string(),
//     }
//     sd.register_method_handler("search", |service, decoder| async {
//         let mut sr = ShardRequest::new();
//         decoder.decode(sr)?;
//         let msg = service.search(sr).await?;
//         Ok::<Shard, failure::Error>(msg)
//     });
// }
