//use mini_redis::{client, Result};
//use mini_redis::{Connection, Frame};
use anyhow::Result;
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
    loop {
        println!("loop cycle");
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { process(socket).await });
    }
    Ok(())
}

async fn process(mut socket: TcpStream) {
    let mut readed_string = String::new();
    let mut stream = BufReader::new(socket);
    let readed = stream.read_line(&mut readed_string).await.unwrap();
    println!("I have readed line : {} ", readed_string);
    test_function().await;
}

async fn test_function() {
    sleep(Duration::from_secs(2)).await;
    println!("ok now");
}
