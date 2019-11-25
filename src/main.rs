extern crate rants;
extern crate uuid;
#[macro_use]
extern crate failure;
// extern crate protoc_rust;
// use protoc_rust::Customize;

mod pb;

use futures::stream::StreamExt;
use pb::rpc::{Request, Response};
use pb::warehouse::{ShipmentRequest, Shipments};
use protobuf::Message;
use rants::{Client, Subject};
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::timer::Timeout;
use uuid::Uuid;

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

    let msg1 = Timeout::new(sub.next(), ACK_TIMEOUT).await?.unwrap();

    match msg1.payload() {
        &[NACK_MSG] => bail!("Retry later! Server sent NACK"),
        &[ACK_MSG] => {},
        _ => bail!("Received unknown message!"),
    }

    // if msg1.payload() != ACK_MSG.as_bytes() {
    //     // TODO: Make this a lot better.
    //     bail!("Did not get an ACK. Why?");
    // }

    let msg2 = Timeout::new(sub.next(), RESPONSE_TIMEOUT).await?.unwrap();

    // let res = std::str::from_utf8(&msg2.payload())?.to_string();
    // println!("The response is: {}", res);

    let mut res = Response::new();
    res.merge_from_bytes(&msg2.payload())?;

    Ok(res)
}

async fn search_shipments(nats: &Client) -> Result<Shipments, failure::Error> {
    let req = ShipmentRequest::new();
    let req_bytes = req.write_to_bytes()?;

    let mut pb_req = Request::new();
    pb_req.set_request_proto(req_bytes);
    pb_req.set_service_name("Warehouse::ShipmentService".to_string());
    pb_req.set_method_name("search".to_string());
    pb_req.set_caller("pb-nats-rs".to_string());

    let sub = "rpc.warehouse.shipment_service.search".to_string();
    let res = send_request_through_nats(&nats, sub, &pb_req).await?;

    let mut shipments = Shipments::new();
    shipments.merge_from_bytes(res.get_response_proto())?;
    //println!("Msg: {}", shipments.get_records()[0].get_guid());
    Ok(shipments)
}

fn main() {
    // protoc_rust::run(protoc_rust::Args {
    //     out_dir: "src/pb",
    //     input: &["src/pb/rpc.proto", "src/pb/warehouse.proto"],
    //     includes: &["src/pb"],
    //     customize: Customize {
    //         ..Default::default()
    //     },
    // }).expect("protoc");
    // println!("Made the proto!");

    let async_main = async {
        let address = "127.0.0.1:4222".parse().unwrap();
        let client = Client::new(vec![address]);
        // Connect to the server
        client.connect().await;

        for _ in 1..3 {
            let shipments = search_shipments(&client).await.unwrap();
            for shipment in shipments.get_records().iter() {
                println!("Shipment guid: {}", shipment.get_guid());
            }
        }

        // send_request_through_nats(&client, sub, payload).await.unwrap();

        client.disconnect().await;
    };

    let runtime = Runtime::new().expect("to create Runtime");
    runtime.spawn(async_main);
    runtime.shutdown_on_idle();
    println!("Done!");
}
