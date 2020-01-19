extern crate rants;
extern crate uuid;
#[macro_use]
extern crate failure;
// extern crate protoc_rust;
// use protoc_rust::Customize;

// mod atlas;
mod pb;
mod rpc;

// use futures::stream::StreamExt;
// use pb::rpc::Request;
use pb::warehouse::{ShipmentRequest, Shipments};
use protobuf::Message;
use rants::Client;
// use std::time::Duration;
// use tokio::runtime::Runtime;
// use tokio::time::timeout;
use tower::Service;
// use uuid::Uuid;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct ShipmentsRpcClient {
    nats: Client,
}

impl rpc::Requestable for ShipmentRequest {
    fn service_name() -> String {
        "Warehouse::ShipmentService".to_string()
    }
    fn method_name() -> String {
        "search".to_string()
    }
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
            let res = rpc::call(&nats, req).await?;
            let mut shipments = Shipments::new();
            shipments.merge_from_bytes(res.get_response_proto())?;
            Ok(shipments)
        })
    }
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
