use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tokio_tungstenite::tungstenite::{
    protocol::Message,
};
use tokio_tungstenite::{connect_async,};
use tokio::net::{TcpListener, TcpStream};
use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};



fn main() {
    println!("Hello, world!");
}

fn factorial(n: u32) -> u32 { if dbg!(n <= 1) { dbg!(1) } else { dbg!(n * factorial(n - 1)) } }


#[test]
fn test_dbg(){
    dbg!(factorial(5));
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);
    let (tx, rx) = unbounded() as ( UnboundedSender<Message>,UnboundedReceiver<Message> );
    let (outgoing, incoming) = ws_stream.split();
    let broadcast_incoming = incoming.try_for_each(|msg| {
        let res = msg.to_text().unwrap();
        println!("Received a message from {}: {}", addr, res);
        if res == "hello"{

            println!("server say hi");
            tx.unbounded_send(Message::text("hi")).unwrap();
            println!("server say hi end");
        }

        future::ok(())
    });
    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    println!("server say hello");
    tx.unbounded_send(Message::text("hello")).unwrap();
    println!("server say hello end");
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);

}
async fn c_client(connect_addr:&str){

    let ws="ws://";
    let url = url::Url::parse(&[ws,connect_addr].join("")).unwrap();
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (write, read) = ws_stream.split();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap();
            let data = data.to_text().unwrap();
            println!("receive data from server:{}",data);
            if data == "hello" {
                println!("client say hi");
                stdin_tx.unbounded_send(Message::text("hi")).unwrap();
                println!("client say hi end");

            }
        })
    };
    pin_mut!(stdin_to_ws, ws_to_stdout);
    println!("client say hello");
    stdin_tx.unbounded_send(Message::text("hello")).unwrap();
    println!("client say hello end");
    future::select(stdin_to_ws, ws_to_stdout).await;
}

#[tokio::test]
async fn websocket_a() -> Result<(), IoError>{
    let connect_addr = "0.0.0.0:8080";
    let try_socket = TcpListener::bind(connect_addr).await;
    let listener = try_socket.expect("Failed to bind");
    tokio::spawn(c_client(connect_addr));
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }

     
    Ok(())
}
