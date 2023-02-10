use rand::distributions::uniform::UniformDuration;
use std::time::Duration;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

pub mod udp;

pub trait EchoServer<T: ToSocketAddrs> {
    fn listen(self, addr: T) -> JoinHandle<()>;
}

pub trait EchoClient<T: ToSocketAddrs> {
    fn hammer(
        self,
        addr: T,
        sockets: usize,
        delay: UniformDuration,
        recorder: UnboundedSender<Duration>,
    ) -> JoinHandle<()>;
}
