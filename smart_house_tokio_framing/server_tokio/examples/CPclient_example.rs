use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{io::Read, net::TcpStream};
mod args;
use args::ClientArgs;
use clap::Parser;
#[path = "../src/ipc_message.rs"]
mod ipc_message;
use ipc_message::*;
fn main() {
    let args = ClientArgs::parse();
    println!("starting client application");
    let mut message_to_server = Message::new(NetMsgType::SendClient);
    match args.enable {
        Some(state) => {
            if state {
                message_to_server.assign_command(Command::TurnOn);
            } else {
                message_to_server.assign_command(Command::TurnOff);
            }
        }
        None => message_to_server.assign_command(Command::GetProperty), // if not en\dis then get info
    }
    message_to_server.assign_devname(args.devname);
    let stream = TcpStream::connect("127.0.0.1:12345");
    if stream.is_err() {
        println!("error connecting to smart house server, check whether it is online, aborting the app...");
        std::process::exit(1);
    }
    let mut buf_read = Vec::<u8>::new();
    let mut stream = stream.unwrap();
    //    let msg_from_server = Message::deserialize_message(&buf_read);
    println!("devname: {}", message_to_server.devname);
    stream.write(&message_to_server.serialize_message());
    //  stream.flush();
    loop {
        println!("reading");
        if let Ok(n) = stream.read(&mut buf_read) {
            if n == 0 {
                println!("readed 0");
                break;
            }
        } else {
            println!("what?");
        }
    }
    println!("readed {:?}", buf_read);
}
