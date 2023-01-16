//use mini_redis::{client, Result};
//use mini_redis::{Connection, Frame};
use anyhow::Result;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::io::BufReader;
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
        tokio::spawn(async move { process(socket, &resource_cpy).await });
    }
    Ok(())
}

async fn process(mut socket: TcpStream, esource: &customtype) {
    let mut readed_string = String::new();
    let mut stream = BufReader::new(socket);
    let readed = stream.read_line(&mut readed_string).await.unwrap();
    println!("I have readed line : {} ", readed_string);
    {
        let mut hm = esource.lock().unwrap();
        for (k, v) in hm.iter() {
            println!("----------->key is {} value is {:?}", k, v);
        }
        hm.insert(readed_string.to_owned(), Bytes::from(readed_string));
    }
    println!("out!");
    //test_function().await;
}

async fn test_function() {
    sleep(Duration::from_secs(2)).await;
    println!("ok now");
}
