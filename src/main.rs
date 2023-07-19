use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    env,
};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt,sink::SinkExt};
use tokio_tungstenite::tungstenite::{
    protocol::Message,
    error::Error as WsError,
};
use tokio_tungstenite::{connect_async,WebSocketStream,};
use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use serde_json::{Result as SResult,value,Value as SValue,json};
use serde::Deserialize;
use dotenv::{dotenv,from_filename};

fn catch_req(data:Option<Result<Message,WsError>> )->SValue {
    let mut end = json!(null);
    let data:Result<Message,WsError> = match data {
        None => return end,
        Some(f) => f,
    };
    let data = match data {
        Err(e) => return end,
        Ok(f) => f,
    };
    let data:&str = match data.to_text(){
        Ok(p) => p,
        Err(e) => return end,
    };
    let mut body:SValue =match serde_json::from_str(data){
        Ok(p) => p,
        Err(e) => return end,
    };
    let path:&SValue = match body.pointer_mut("/path"){
        None => return end,
        Some(p)=> p,
    };
    end = json!({});
    let message:&SValue = match body.pointer_mut("/message"){
        None => return end,
        Some(p)=> p,
    };
    let message = message.clone();
    let message:&str = match message.as_str(){
        None => return end,
        Some(p) => p,
    };
    match serde_json::from_str(message){
        Ok(p) => p,
        Err(e) => return end,
    }
}
fn catch_err(data:Option<Result<Message,WsError>> )->SValue {
    let end = json!(null);
    let data:Result<Message,WsError> = match data {
        None => return end,
        Some(f) => f,
    };
    if data.is_ok(){
        let data = data.unwrap(); 
        let data:&str = match data.to_text(){
            Ok(p) => p,
            Err(e) => return end,
        };
        println!("catch err ok:::::{:?}",data);
        let mut body:SValue =match serde_json::from_str(data){
            Ok(p) => p,
            Err(e) => return end,
        };
        //println!("---------------------------3329");
        let message:&SValue = match body.pointer_mut("/message"){
            None => &body,
            Some(p)=> p,
        };
        let message = message.clone();
        //println!("---------------------------3330");
        if message.is_string(){

            let message:&str = match message.as_str(){
                None => return end,
                Some(p) => p,
            };
            //println!("---------------------------3331");
            body=match serde_json::from_str(message){
                Ok(p) => p,
                Err(e) => return end,
            };
        }
        let code:&SValue = match body.pointer_mut("/result/code"){
            None => return end,
            Some(p)=> p,
        };
        //println!("---------------------------3332");
        let code:&str = match code.as_str(){
            None => return end,
            Some(p) => p,
        };
        //println!("{:?}",code);
        //println!("---------------------------3333");
        if code == "0000"{
            //println!("---------------------------3334");
            let result:SValue = match body.pointer("/result/body"){
                None => return json!({}),
                Some(p) => return p.clone(),

            };
            //if result.is_none(){
            //    return json!({});
            //}else{
            //    //let mut result = result.unwrap();
            //    //return result.clone();   
            //    return result;
            //}
        }
    }
    end
}

//fn catch_err(data:Option<Message> )->SResult<SValue> {
//    let data = match data.unwrap(){
//        Err(e) => return None,
//        Ok(f) => f,
//    };
//    let data = match data.unwrap(){
//        Err(e) => return None,
//        Ok(f) => f,
//    };
//    let data:SValue =match serde_json::from_str(data).unwrap(){
//        Err(e) => return None,
//        Ok(f) => f,
//    }; 
//    let message = data.pointer("/message");
//    if message != None {
//        let msg = match message.unwrap() {
//            Err(e) => return None,
//            Ok(f) => f,
//        };
//        let msg = match msg.as_str().unwrap(){
//            Err(e) => return None,
//            Ok(f) => f,
//        };
//        data = match serde_json::from_str(msg).unwrap(){
//            Err(e) => return None,
//            Ok(f) => f,
//        };
//    }
//    let res = match data.pointer("/result/code").unwrap(){
//            Err(e) => return None,
//            Ok(f) => f,
//
//    };
//    let res = match res.as_str().unwrap(){
//            Err(e) => return None,
//            Ok(f) => true,
//    };
//    Ok(data)
//}


#[tokio::test]
async fn test_plan_a(){
    from_filename(".env.local").ok();
    println!("run plan a");
    let sign_data = r#"
    {
        "message":"sign",
        "serviceId":"1001",
        "requestId":"10009_1586-test0001"
    }"#;
    
    let host = env::var("HOST_TEST_WS").unwrap_or("".to_string());
    let host = &["ws://",&host,"/dws/ddPosClient"].join("");
    println!("{:?}",host);
    let url = url::Url::parse(host).unwrap();
    //let (mut ws_stream, response) 
    let result= connect_async(url).await;
    if result.is_err() {
        println!("err url coonect");
        return 
    }
    let (mut ws_stream, response) = result.unwrap();
    //expect("Failed to connect");
    let (parts, body) = response.into_parts();
    println!("{:?}",parts.status);
    println!("{:?}",parts.version);
    println!("{:?}",parts.headers);
    println!("{:?}",parts.extensions);
    ws_stream.send(Message::Text(sign_data.to_string())).await;
    //let data = ws_stream.next().await;
    //let res = catch_err(data);
    let mut sign = true;
    while sign {
        tokio::select!{
            data = ws_stream.next() =>{
                let res = catch_err(data);
                if res.is_object(){
                    println!("sign in ok");
                    sign = false;
                }else{
                    println!("sign in err");
                }
            }
            _ = time::sleep(time::Duration::from_secs(4)) => {
                println!("select 4s");
            }
        };

    }
    let ping_data = r#"{
        "message":"heart",
        "serviceId":"1005",
        "requestId":"10009_1586-test0001"
    }"#;
    let ping_data = ping_data.to_string();
    let (mut write,mut read) = ws_stream.split();
    loop {
        tokio::select!{
            data = async {
                read.next().await
            } =>{
                let res = catch_req(data);
                println!("{:?}",res);

            }
            _ = async {
                time::sleep(time::Duration::from_secs(4)).await;
                write.send(Message::Text(ping_data.clone())).await;
                // Help the rust type inferencer out
                // Ok::<_, io::Error>(())
            } => {
                println!("after 4s and send msg");
            }
        };
    } 
    //let data = data.unwrap();
    //let data = data.unwrap();
    //let data = data.to_string();
    //let mut data:SValue = serde_json::from_str(&data).unwrap(); 
    //let message = data.pointer("/message");
    //if message != None{
    //    let aaa = message.unwrap().as_str().unwrap();
    //    println!("{:?}",aaa);
    //    data = serde_json::from_str(aaa).unwrap();
    //}
    //println!("{:?}",data.pointer("/code"));
}



#[test]
fn test_option_a(){
    println!("test option a");
    let abc = Some(3);
    let abc = abc.unwrap();
    println!("{:?}",abc);

    
let optional = None;
check_optional(optional);

let optional = Some(Box::new(9000));
check_optional(optional);

fn check_optional(optional: Option<Box<i32>>) {
    match optional {
        Some(p) => println!("has value {p}"),
        None => println!("has no value"),
    }
}

fn check_num(abc: i32)-> Option<i32>{
    if abc > 1000{
        return Some(abc)
    } 
    return None
}
println!("{:?}",check_num(343242));
println!("{:?}",check_num(242));
println!("{:?}",check_num(32));
println!("{:?}",check_num(343242));



}



#[test]
fn test_result_a(){
    let res = (||{
        println!("test result a");
        Ok(())
    })().unwrap_or_else(|_err:String|{
        
    });
}


#[test]
fn test_match_return(){
    fn op_panic(num:Option<i32>) -> Result<i32, String>{
        let res = num.unwrap();
        if res == 3{
            return Ok(res);
        }
        if res == 4{
            return Err("happend".to_string());
        }
        panic!("last msg") 
    }

    println!("op_panic 3 is {:?}",op_panic(Some(3)));
    //thread 'test_match_return' panicked at 'last msg', src/main.rs:155:9
    //println!("op_panic 4 is {:?}",op_panic(Some(4)));
    let panics = std::panic::catch_unwind(|| op_panic(Some(5))).is_err(); 
    if panics {
        println!("panic catch by std::panic::catch_unwind");
    }
    println!("op_panic 4 is {:?}",op_panic(Some(4)));
    //thread 'test_match_return' panicked at 'called `Result::unwrap()` on an `Err` value: "happend"',
    //let res = op_panic(Some(4)).unwrap();
    let res = match op_panic(Some(4)) {
        Err(p) => 0,
        Ok(p) => p,
    };
    
    println!("op_panic 4  unwrap is {:?}",res);
    let res = match op_panic(Some(3)) {
        Err(p) => 0,
        Ok(p) => p,
    };
    
    println!("op_panic 3  unwrap is {:?}",res);


    fn opfn(num:i32) -> Result<i32,()>{
        let aa = match num {
            11 => return Ok(1),
            12 => return Ok(2),
            13 => return Ok(4),
            14 => return Ok(4),
            oth => Ok(oth-101),
        };
        println!("run oth------------------------------");
        aa
    }
    println!("11 is {:?}",opfn(11));
    println!("12 is {:?}",opfn(12));
    println!("13 is {:?}",opfn(13));
    println!("14 is {:?}",opfn(14));
    println!("1 is {:?}",opfn(1));
    println!("2 is {:?}",opfn(2));
    println!("3 is {:?}",opfn(3));
    println!("332 is {:?}",opfn(332));
}



#[tokio::test]
async fn test_select_a() {
    while true {
        tokio::select!{
            _ = time::sleep(time::Duration::from_secs(4)) => {
                println!("only show 4");
            }
            _ = time::sleep(time::Duration::from_secs(5)) => {
                println!("never show 5");
            }
        };
    }
}
