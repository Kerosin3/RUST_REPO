use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
#[path = "../src/ipc_message.rs"]
mod ipc_message;
use ipc_message::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut rw_stream = TcpStream::connect("127.0.0.1:12345").await.unwrap();
    let mut buf_read = Vec::<u8>::new();
    rw_stream.read_to_end(&mut buf_read).await.unwrap();
    let msg_from_server = Message::deserialize_message(&buf_read);
    println!("got device: {}", msg_from_server.devname);
    {
        // ackn
        if msg_from_server.devname == "default" {
            rw_stream
                .write_all(&Message::new(NetMsgType::SendClient).serialize_message())
                .await
                .unwrap();
            rw_stream.flush().await.unwrap();
            println!("written!");
        }
    }

    Ok(())
}
