#![allow(unused_imports)]
//use lib_shouse::home::home::home::*;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::io::Read;
use std::{io::Write, net::TcpStream};
mod args;
use args::ClientArgs;
use clap::Parser;

#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum IpcState {
    Get_state = 0,
    Set_state,
    Turn_on,
    Turn_off,
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct IpcMessage {
    state: IpcState,
    devname: String,
}

impl IpcMessage {
    fn new() -> Self {
        Self {
            state: IpcState::Turn_off,
            devname: "defalt name".to_string(),
        }
    }
    fn set_devname(&mut self, name: &str) {
        self.devname = name.to_owned();
    }
    fn set_state(&mut self, state: IpcState) {
        self.state = state;
    }
}

fn main() {
    let args = ClientArgs::parse();
    println!("starting client application");
    let mut ipc_msg = IpcMessage::new();
    ipc_msg.set_devname(&args.devname); //assign devname
    match args.enable {
        Some(state) => {
            if state {
                ipc_msg.set_state(IpcState::Turn_on); // enable dev
            } else {
                ipc_msg.set_state(IpcState::Turn_off); // disable dev
            }
        }
        None => ipc_msg.set_state(IpcState::Get_state), // if not en\dis then get info
    }
    let msg = bincode::serialize(&ipc_msg).unwrap(); // serialize msg
    let mut buf: Vec<u8> = Vec::with_capacity(4096);
    let stream = TcpStream::connect("127.0.0.1:12345");
    if stream.is_err() {
        println!("error connecting to smart house server, check whether it is online, aborting the app...");
        std::process::exit(1);
    }
    let mut stream = stream.unwrap();
    stream.write_all(&msg).expect("failed to send data");
    let mut reader = BufReader::new(&stream); // create reader
    reader.read_to_end(&mut buf).unwrap();
    let ipc_msg_from_server: IpcMessageToClient = bincode::deserialize(&buf).unwrap();
    if let Some(error_serv) = ipc_msg_from_server.errors {
        println!("error from server: {error_serv}");
    } else if ipc_msg_from_server.turning.is_some() {
        println!(
            "device {} is now enabled: [{}]",
            ipc_msg_from_server.devname,
            ipc_msg_from_server.state.unwrap()
        );
    } else {
        println!(
            "Retrieved info from the home server, devname: {}, state: {}, info: {}",
            ipc_msg_from_server.devname,
            ipc_msg_from_server.state.unwrap(),
            ipc_msg_from_server.property.unwrap()
        );
    }
    // println!(
    //     "received answer from the server: {}",
    //     String::from_utf8_lossy(&buf)
    // );
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub struct IpcMessageToClient {
    devname: String,
    property: Option<String>,
    state: Option<bool>,
    turning: Option<()>,
    room_name: Option<String>,
    errors: Option<String>,
}
