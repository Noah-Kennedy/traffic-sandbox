use rand::distributions::uniform::{UniformDuration, UniformSampler};
use std::time::Duration;
use tokio::sync::mpsc::unbounded_channel;
use traffic_server_core::EchoClient;

#[tokio::main]
async fn main() {
    let client = traffic_server_core::udp::UdpEchoClient;

    let (tx, mut rx) = unbounded_channel();

    client.hammer(
        "0.0.0.0:1776",
        512,
        UniformDuration::new_inclusive(Duration::ZERO, Duration::from_millis(5)),
        tx,
    );

    let mut data = Vec::new();

    for _ in 0..1000 {
        let d = rx.recv().await.unwrap();
        data.push(d)
    }

    for d in data {
        println!("{} us", d.as_micros());
    }
}
