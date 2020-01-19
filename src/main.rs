extern crate rants;
extern crate uuid;
#[macro_use]
extern crate failure;
// extern crate protoc_rust;
// use protoc_rust::Customize;

// mod atlas;
mod pb;

use futures::stream::StreamExt;
use pb::rpc::{Request, Response};
use pb::warehouse::{ShipmentRequest, Shipments};
use protobuf::Message;
use rants::{Client, Subject};
use std::time::Duration;
//use tokio::runtime::Runtime;
use tokio::time::timeout;
use tower::Service;
use uuid::Uuid;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct ShipmentsRpcClient {
    nats: Client,
}

impl ShipmentsRpcClient {
    fn new(nats: Client) -> ShipmentsRpcClient {
        ShipmentsRpcClient { nats: nats }
    }
}

impl Service<ShipmentRequest> for ShipmentsRpcClient {
    type Response = Shipments;
    type Error = failure::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: ShipmentRequest) -> Self::Future {
        let nats = self.nats.clone();

        Box::pin(async move {
            let req_bytes = req.write_to_bytes()?;
            let mut pb_req = Request::new();
            pb_req.set_request_proto(req_bytes);
            pb_req.set_service_name("Warehouse::ShipmentService".to_string());
            pb_req.set_method_name("search".to_string());
            pb_req.set_caller("pb-nats-rs".to_string());

            let sub = build_subject_from_request(&pb_req);
            let res = send_request_through_nats(&nats, sub, &pb_req).await?;

            let mut shipments = Shipments::new();
            shipments.merge_from_bytes(res.get_response_proto())?;

            Ok(shipments)
        })
    }
}

// TODO: Move this
pub trait ServiceDescription {
    fn service_name() -> String;
    fn method_name() -> String;
}

// TODO: Move this
impl ServiceDescription for ShipmentRequest {
    fn service_name() -> String {
        return "Warehouse::ShipmentService".to_string();
    }
    fn method_name() -> String {
        return "search".to_string();
    }
}

const ACK_TIMEOUT: Duration = Duration::from_secs(5);
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(60);
const ACK_MSG: u8 = '\u{0001}' as u8;
const NACK_MSG: u8 = '\u{0002}' as u8;

fn new_inbox() -> Subject {
    Uuid::new_v4().to_string().parse().unwrap()
}

async fn send_request_through_nats(
    nats: &Client,
    subject: String,
    req: &Request,
) -> Result<Response, failure::Error> {
    let subject = subject.parse()?;
    let inbox = new_inbox();
    let (sid, mut sub) = nats.subscribe(&inbox, 2).await?;
    nats.unsubscribe_with_max_msgs(sid, 2).await?;

    let payload = req.write_to_bytes().unwrap();
    nats.publish_with_reply(&subject, &inbox, &payload).await?;

    let msg1 = timeout(ACK_TIMEOUT, sub.next()).await?.unwrap();
    if msg1.payload() == [NACK_MSG] {
        bail!("Retry later! Server sent NACK");
    }
    let msg2 = timeout(RESPONSE_TIMEOUT, sub.next()).await?.unwrap();

    let res = match (msg1.payload(), msg2.payload()) {
        ([ACK_MSG], bytes) | (bytes, [ACK_MSG]) => {
            let mut res = Response::new();
            res.merge_from_bytes(&bytes)?;
            res
        }
        _ => bail!("No ACK msg received!: (msg1: {:?}, msg2: {:?})", msg1, msg2),
    };
    Ok(res)
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
fn build_subject_from_request(req: &Request) -> String {
    let service = req.get_service_name();
    let service = service.replace("::", ".");
    let service = to_snake_case(service);
    let method = req.get_method_name().to_string();
    let method = to_snake_case(method);
    format!("rpc.{}.{}", service, method)
}

use tower::{ServiceBuilder, ServiceExt};

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:4222".parse().unwrap();
    let client = Client::new(vec![address]);
    // Connect to the server
    client.connect().await;

    let rpc_client = ServiceBuilder::new()
        .buffer(100)
        .concurrency_limit(100)
        .service(ShipmentsRpcClient::new(client.clone()));

    let futures: Vec<_> = (1..100)
        .into_iter()
        .map(|i| {
            let mut rpc_client = rpc_client.clone();
            tokio::spawn(async move {
                let mut req = ShipmentRequest::new();
                let mut args = protobuf::RepeatedField::new();
                args.push(i.to_string());
                req.set_guid(args);
                match rpc_client.ready().await {
                    Ok(_) => {
                        println!("Got response for: {}", i);
                        match rpc_client.call(req).await {
                            Ok(shipments) => {
                                //let shipments = search_shipments(&client).await.unwrap();
                                for shipment in shipments.get_records().iter() {
                                    println!("Shipment guid: {}", shipment.get_guid());
                                }
                            }
                            Err(err) => println!("Error: {}", err),
                        }
                    }
                    Err(err) => println!("Error: {}", err),
                };
            })
        })
        .collect();

    // futures.iter().map(|f| f.await);

    futures::future::join_all(futures).await;

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
