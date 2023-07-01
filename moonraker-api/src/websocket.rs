use crate::MoonrakerMsg;
use anyhow::Result;
use fastwebsockets::{FragmentCollector, Frame};
use hyper::{
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Body, Request,
};
use std::future::Future;
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

type MoonrakerMsgRx = tokio::sync::mpsc::UnboundedReceiver<MoonrakerMsg>;
type MoonrakerMsgTx = tokio::sync::mpsc::UnboundedSender<MoonrakerMsg>;

pub async fn connect(moonraker_api_url: &str) -> Result<(MoonrakerMsgTx, MoonrakerMsgRx)> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<MoonrakerMsg>();
    let (out_tx, out_rx) = tokio::sync::mpsc::unbounded_channel::<MoonrakerMsg>();

    let moonraker_api_url = moonraker_api_url.to_string();

    tokio::spawn(async move {
        loop {
            println!("DBG: Attempting to connect to moonraker websocket...");
            let res = ws_connection(&moonraker_api_url, &out_tx, &mut rx).await;
            if let Err(e) = res {
                println!("DBG Error: {}", e);
            }

            println!("DBG: Reconnecting in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    Ok((tx, out_rx))
}

async fn ws_connection(
    moonraker_api_url: &str,
    tx: &MoonrakerMsgTx,
    rx: &mut MoonrakerMsgRx,
) -> Result<()> {
    let mut ws = connect_to_ws(moonraker_api_url).await?;
    println!("DBG: Connected to moonraker websocket");

    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                let json = msg.to_json();
                let payload = json.as_bytes();

                ws.write_frame(Frame::text(payload.into())).await.unwrap();
            }
            msg = ws.read_frame() => {
                if let Err(e) = msg {
                    println!("DBG: Error while reading frame: {}", e);
                    return Err(e.into());
                }

                let payload = msg.unwrap().payload;
                let json = std::str::from_utf8(&payload).unwrap();

                let msg = MoonrakerMsg::from_json(json);
                if let Ok(msg) = msg {
                    tx.send(msg).unwrap();
                } else {
                    //println!("DBG: {}", msg.err().unwrap());
                    //println!("DBG: Received: {}", json);
                }
            }
        }
    }
}

async fn connect_to_ws(moonraker_api_url: &str) -> Result<FragmentCollector<Upgraded>> {
    let stream = TcpStream::connect(moonraker_api_url).await?;

    let req = Request::builder()
        .method("GET")
        .uri("/websocket")
        .header("Host", moonraker_api_url)
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
