//use mini_redis::{client, Result};
//use mini_redis::{Connection, Frame};
use anyhow::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::AsyncWrite;
use tokio::io::BufReader;
use tokio::io::BufStream;
use tokio::io::{AsyncBufReadExt, BufWriter};
use tokio::io::{AsyncRead, ReadHalf, WriteHalf};
use tokio::time::sleep;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
type customtype = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
    let mut resource: customtype = Arc::new(Mutex::new(HashMap::new()));
    loop {
        println!("loop cycle");
        let (socket, _) = listener.accept().await.unwrap();
        let resource_cpy = Arc::clone(&resource);
        //        tokio::spawn(async move { process(socket, &resource_cpy).await });
        tokio::spawn(async move { simplev2(socket).await });
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpcMessage {
    state: BytesMut,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    TurnOn,
    TurnOff,
    GetProperty,
    EstablishConn,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum NetMsgType {
    SendClient,
    SendServer,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    devname: String,
    msg_type: NetMsgType,
    command: Command,
    info: Option<String>,
}

async fn simplev2(mut socket: TcpStream) {
    let mut rw_stream = BufStream::new(socket);
    let mut buf_read = BytesMut::new();
    rw_stream.write_all(b"hello!\r\n").await;
    rw_stream.flush().await.unwrap();
    loop {
        if let Ok(vec_ret) = rw_stream.fill_buf().await {
            if vec_ret.is_empty() {
                println!("finishing sending");
                break;
            }
            buf_read.extend_from_slice(vec_ret);
            rw_stream.consume(buf_read.len());
            println!("readed {:?}", buf_read);
            let message = IpcMessage {
                state: buf_read.clone(),
            };
            buf_read.clear();
            let ser_msg = bincode::serialize(&message).unwrap();
            println!("msg: {:?}", ser_msg);
            rw_stream.write_all(&ser_msg).await.unwrap();
            rw_stream.flush().await.unwrap();
            println!("here");
        } else {
            println!("exiting due to IO error");
            break;
        }
    }
    //    rw_stream.shutdown().await.unwrap();//error??
}

/*
async fn simple(mut socket: TcpStream) {
    let (mut rd, mut wr) = socket.split(); // split read and write
    let mut stream_write = BufWriter::new(wr);
    let mut stream_read = BufReader::new(rd);
    // say hello
    //   write_back(&mut stream_write).await;
    let mut buf_from_client = BytesMut::new();
    loop {
        if stream_read.read_buf(&mut buf_from_client).await.unwrap() == 0 {
            println!("shutdown the connection");
            break;
        }
        println!("we readed {:?}", buf_from_client);
        buf_from_client.clear();
    }
}
/*
async fn write_back(wr_buf: &mut BufWriter<WriteHalf<'_>>) {
    wr_buf
        .write_all(b"hello, say what do you want?\r\n")
        .await
        .unwrap();
    wr_buf.flush().await.unwrap();
}
*/
async fn process(mut socket: TcpStream, esource: &customtype) {
    let mut readed_string = String::new();
    let (mut rd, mut wr) = socket.split();
    let mut stream = BufReader::new(rd);
    let mut stream_write = BufWriter::new(wr);
    stream_write
        .write_all_buf(&mut Bytes::from("hello!".to_string()))
        .await
        .is_ok();
    let readed = stream.read_line(&mut readed_string).await.unwrap();
    println!("I have readed line : {} ", readed_string);
    let mut buf_back = Bytes::from(readed_string.to_owned());
    if stream_write.write_all_buf(&mut buf_back).await.is_ok() {
        println!("sende");
    }
    {
        let mut hm = esource.lock().unwrap();
        for (k, v) in hm.iter() {
            println!("----------->key is {} value is {:?}", k, v);
        }
        hm.insert(
            readed_string.to_owned(),
            Bytes::from(readed_string.to_owned()),
        );
    }
    println!("out!");
    //test_function().await;
}

async fn test_function() {
    sleep(Duration::from_secs(2)).await;
    println!("ok now");
}
*/
