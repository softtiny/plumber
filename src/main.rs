use std::{
   // collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    borrow::Cow,
};
use futures_util::{
    future, 
    pin_mut, 
    stream::{TryStreamExt, StreamExt,},
    sink::{SinkExt,},
};
use tokio_tungstenite::tungstenite::{
    protocol::{ 
        Message, 
        frame::CloseFrame,
        frame::coding::CloseCode,
    }
};
use tokio_tungstenite::{connect_async,};
use tokio::net::{TcpListener, TcpStream};
use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use serde_json::{Result as SResult,value,Value as SValue};
use serde::Deserialize;


static URL: &'static str = "0.0.0.0:8000";


#[test]
fn test_base(){
    println!("test base ok");
}

async fn c_client(connect_addr:&str){

    let ws="ws://";
    let url = url::Url::parse(&[ws,connect_addr].join("")).unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    //ws_stream.then(|x| async {
    //    let msg = x.unwrap();
    //    println!("{}",msg);
    //});
    let res = ws_stream.next().await;
    let res = res.unwrap();
    println!("{:?}","ca0001");
    let res =  res.unwrap();
    println!("{:?}",res.to_string());
    println!("{:?}","ca0002");
    
    ws_stream.send(Message::text(res.to_string()+";;;111")).await;
    let res = ws_stream.next().await;
    let res = ws_stream.next().await;
}

async fn handle_connection(raw_stream: TcpStream,name: &str) {
    let mut ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    ws_stream.send(Message::text("hello ".to_string() + name)).await;
    let mut count = 0;
    while let res = ws_stream.next().await {
        if res.is_none() {
            println!("break when none happen");
            break;
        }
        let result = res.unwrap();
        
        let result = result.unwrap_or(Message::text("402"));
        
        println!("{:?}","s0001".to_string()+name);
        let result_res = result.into_text().unwrap();
        if result_res == "402" {
            println!("402 ...............");
        }else{
            println!("{:?}",result_res);

        }

        //let result = result.unwrap();
        //if result.is_close() {
        //    println!("close from client");
        //    //break;
        //}else{
        //    println!("{:?}","s0001".to_string()+name);
        //    println!("{:?}",result);
        //    println!("{:?}","s0002".to_string()+name);
        //}
    }
}

#[tokio::test]
async fn test_plan_a(){
    println!("test plan a run:");
    let try_socket = TcpListener::bind(URL).await;
    let listener = try_socket.expect("Failed to bind");
    let mut names = vec!("a","b");
    tokio::spawn(c_client(URL));
    tokio::spawn(c_client(URL));
    while let Ok((stream,addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream,names.remove(0)));
    }
}


async fn clientb( href: &str ){
    let ws="ws://";
    let url = url::Url::parse(&[ws,href].join("")).unwrap();
    let (mut ws_stream, _) = connect_async(url.clone()).await.expect("Failed to connect");
    ws_stream.send(Message::Text("client_a:hello".to_string())).await;
    let close = CloseFrame { code: CloseCode::Away, reason: Cow::from("close") };
    ws_stream.close(Some(close)).await;
    //let ws_stream = 0;
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    ws_stream.send(Message::Text("client_b:hello".to_string())).await;
    let ws_stream = 0;
}

#[tokio::test]
async fn test_plan_b(){
    println!("test plan a run:");
    let try_socket = TcpListener::bind(URL).await;
    let listener = try_socket.expect("Failed to bind");
    let mut names = vec!("a","b");
    tokio::spawn(clientb(URL));
    while let Ok((stream,addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream,names.remove(0)));
    }
}





async fn clientb__bak(href:&str){

    let ws="ws://";
    let url = url::Url::parse(&[ws,href].join("")).unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (write, read) = ws_stream.split();
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);

    pin_mut!(stdin_to_ws);
}

