use super::pb::rpc::{Request as RpcRequest, Response as RpcResponse};
use futures::stream::StreamExt;
// use pb::warehouse::{ShipmentRequest, Shipments};
use protobuf::Message;
use rants::{Client, Subject};
use std::time::Duration;
//use tokio::runtime::Runtime;
use tokio::time::timeout;
// use tower::Service;
use uuid::Uuid;

// use std::future::Future;
// use std::pin::Pin;
// use std::task::{Context, Poll};

const ACK_TIMEOUT: Duration = Duration::from_secs(5);
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(60);
const ACK_MSG: u8 = '\u{0001}' as u8;
const NACK_MSG: u8 = '\u{0002}' as u8;

fn new_inbox() -> Subject {
    Uuid::new_v4().to_string().parse().unwrap()
}

pub async fn send_request_through_nats(
    nats: &Client,
    subject: String,
    req: &RpcRequest,
) -> Result<RpcResponse, failure::Error> {
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
            let mut res = RpcResponse::new();
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
pub fn build_subject_from_request(req: &RpcRequest) -> String {
    let service = req.get_service_name();
    let service = service.replace("::", ".");
    let service = to_snake_case(service);
    let method = req.get_method_name().to_string();
    let method = to_snake_case(method);
    format!("rpc.{}.{}", service, method)
}

pub trait Requestable {
    fn service_name() -> String;
    fn method_name() -> String;
}

pub async fn call<R>(nats: &Client, req: R) -> Result<RpcResponse, failure::Error>
where
    R: Requestable + ::protobuf::Message,
{
    let req_bytes = req.write_to_bytes()?;
    let mut pb_req = RpcRequest::new();
    pb_req.set_request_proto(req_bytes);
    pb_req.set_service_name(R::service_name());
    pb_req.set_method_name(R::method_name());
    pb_req.set_caller("pb-nats-rs".to_string());
    let sub = build_subject_from_request(&pb_req);
    send_request_through_nats(&nats, sub, &pb_req).await
}
