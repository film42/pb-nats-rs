pb-nats-rs
==========

This is a simple rpc client impl to work with protobuf-nats. This uses tower to add service layers.

Do not use in production. This is for learning.

Example:

```rust
#[tokio::main]
async fn main() {
    let address = "127.0.0.1:4222".parse().unwrap();
    let client = Client::new(vec![address]);
    client.connect().await;

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
    let mut res: Shipments = sc.search(req).await.unwrap();
    let mut shipment: Shipment = res.take_records().pop().unwrap();
    println!("Response: {:?}", shipment);
}
```

This will output something like:

```
Started!
Response: guid: "165e8b52-c3bf-4871-82b6-b2d0db30936a" address: "123 LAME ST" price: 100 package_guid: "16b37958-8ed4-4756-8e1f-f6440522c57d"
```

License: MIT
