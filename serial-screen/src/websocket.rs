use crate::MOONRAKER_API_URL;
use anyhow::Result;
use fastwebsockets::FragmentCollector;
use hyper::{
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Body, Request,
};
use std::{future::Future, pin::Pin};
use tokio::net::TcpStream;

struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}

pub async fn websocket_connection_loop() -> Result<()> {
    tokio::spawn(async {
        loop {
            println!("Attempting to connect to moonraker websocket...");
            let res = ws_connection().await;
            if let Err(e) = res {
                println!("Error: {}", e);
            }

            println!("Reconnecting in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    Ok(())
}

async fn ws_connection() -> Result<()> {
    let mut ws = connect_to_ws().await?;

    loop {
        let msg = ws.read_frame().await;
        if let Ok(msg) = msg {
            println!("Received: {:?}", msg.payload);
        }
    }
}

async fn connect_to_ws() -> Result<FragmentCollector<Upgraded>> {
    let stream = TcpStream::connect(MOONRAKER_API_URL).await?;

    let req = Request::builder()
        .method("GET")
        .uri("/websocket")
        .header("Host", MOONRAKER_API_URL)
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        .header(
            "Sec-WebSocket-Key",
            fastwebsockets::handshake::generate_key(),
        )
        .header("Sec-WebSocket-Version", "13")
        .body(Body::empty())?;

    let (ws, _) = fastwebsockets::handshake::client(&SpawnExecutor, req, stream).await?;
    Ok(FragmentCollector::new(ws))
}
