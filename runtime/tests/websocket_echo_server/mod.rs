// Based on https://github.com/snapview/tokio-tungstenite/blob/a8d9f1983f1f17d7cac9ef946bbac8c1574483e0/examples/echo-server.rs

use std::{io, net::SocketAddr};

use futures_util::{future, StreamExt, TryStreamExt};
use log::debug;
use tokio::net::{TcpListener, TcpStream};

pub struct WebSocketEchoServer {
    listener: TcpListener,
}

impl WebSocketEchoServer {
    pub async fn create() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        Ok(Self { listener })
    }

    pub fn port(&self) -> io::Result<u16> {
        match self.listener.local_addr()? {
            SocketAddr::V4(addr) => Ok(addr.port()),
            addr => panic!("unexpected WebSocketEchoServer address type: {addr:?}"),
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        while let Ok((stream, _)) = self.listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
        Ok(())
    }
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    debug!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    debug!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}
