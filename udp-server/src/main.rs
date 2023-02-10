use traffic_server_core::EchoServer;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let server = traffic_server_core::udp::UdpEchoServer { workers: 1 };

    server.listen("0.0.0.0:1776").await.unwrap();
}
