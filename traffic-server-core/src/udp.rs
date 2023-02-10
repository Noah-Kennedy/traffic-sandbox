use crate::{EchoClient, EchoServer};
use rand::distributions::uniform::{UniformDuration, UniformSampler};
use std::time::{Duration, Instant};
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::{JoinHandle, JoinSet};

pub struct UdpEchoServer {
    pub workers: usize,
}

pub struct UdpEchoClient;

impl<T> EchoClient<T> for UdpEchoClient
where
    T: ToSocketAddrs + Clone + Send + 'static,
{
    fn hammer(
        self,
        addr: T,
        sockets: usize,
        delay: UniformDuration,
        recorder: UnboundedSender<Duration>,
    ) -> JoinHandle<()> {
        let mut set = JoinSet::new();

        for _ in 0..sockets {
            set.spawn(run_client_worker(
                addr.clone(),
                delay.clone(),
                recorder.clone(),
            ));
        }

        tokio::spawn(async move { while set.join_next().await.is_some() {} })
    }
}

impl<T> EchoServer<T> for UdpEchoServer
where
    T: ToSocketAddrs + Clone + Send + 'static,
{
    fn listen(self, addr: T) -> JoinHandle<()> {
        let mut set = JoinSet::new();

        for _ in 0..self.workers {
            set.spawn(run_server_worker(addr.clone()));
        }

        tokio::spawn(async move { while set.join_next().await.is_some() {} })
    }
}

async fn run_client_worker<T>(
    addr: T,
    delay: UniformDuration,
    recorder: UnboundedSender<Duration>,
) -> Duration
where
    T: ToSocketAddrs,
{
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.connect(addr).await.unwrap();

    let mut rng = rand::rngs::OsRng;

    loop {
        let pause = delay.sample(&mut rng);

        tokio::time::sleep(pause).await;

        let mut buf = [0; 1024];

        if let Ok(len) = socket.send(&mut buf).await {
            let timer = Instant::now();

            if socket.recv(&mut buf[..len]).await.is_ok() {
                let elapsed = timer.elapsed();

                let _ = recorder.send(elapsed);
            }
        }
    }
}

async fn run_server_worker<T>(addr: T)
where
    T: ToSocketAddrs,
{
    let socket = UdpSocket::bind(addr).await.unwrap();

    loop {
        let mut buf = [0; 2048];

        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            let _ = socket.send_to(&buf[..len], addr).await;
        }
    }
}
