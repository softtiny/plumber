use tokio::net::{TcpListener, TcpStream};
use std::io;
use chrono::{DateTime, Local};
use futures_util::future;
use futures_util::stream::{TryStreamExt,StreamExt};
use std::env;

fn main () {
    println!("lll");
}

#[test]
fn test_ok () {
    println!("run test ok!");
}

async fn handle_connect(socket: tokio::net::TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(socket)
        .await
        .expect("Error during the websocket handshake occurred");
    let (outgoing, incoming) = ws_stream.split();
    let broadcast_incoming = incoming.try_for_each(|msg| {
        let res = msg.to_text().unwrap();
        println!("{}",res);
        future::ok(())
    });
    println!("0000000000000000000");
}

#[tokio::test]
async fn test_bind_port () -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let listener = TcpListener::bind("0.0.0.0:3128").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let now: DateTime<Local> = Local::now();
        println!("Current date and time: {}", now);
        tokio::spawn(async move {
            handle_connect(socket).await;
        });
    }
}