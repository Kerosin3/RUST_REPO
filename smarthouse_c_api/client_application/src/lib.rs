#![allow(unused_imports)]
//use lib_shouse::home::home::home::*;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::io::Read;
use std::{io::Write, net::TcpStream};
mod args;
use args::ClientArgs;
use clap::Parser;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
#[no_mangle]
pub extern "C" fn enable_termometer(enable: bool, get_info: bool, arg_s: *const c_char) -> CString {
    println!("starting client application");
    let c_str = unsafe {
        assert!(!arg_s.is_null());
        CStr::from_ptr(arg_s)
    };
    let Ok(string_from_c) = c_str.to_str() else {
        let answer = format!("error reading device name");
        return CString::new(answer).unwrap()
    };
    let mut ipc_msg = IpcMessage::new();
    ipc_msg.set_devname(string_from_c); //assign devname
    match enable {
        true => {
            ipc_msg.set_state(IpcState::Turn_on); // enable dev
        }
        false => {
            ipc_msg.set_state(IpcState::Turn_off); // enable dev
        }
    }
    if get_info {
        ipc_msg.set_state(IpcState::Get_state);
    };
    println!("Getting termo information, enabling/disabling is ignored");
    let msg = bincode::serialize(&ipc_msg).unwrap(); // serialize msg
    let mut buf: Vec<u8> = Vec::with_capacity(4096);
    let stream = TcpStream::connect("127.0.0.1:12345");
    if stream.is_err() {
        let answer = format!("There is no server running");
        return CString::new(answer).unwrap();
    }
    let mut stream = stream.unwrap();
    stream.write_all(&msg).expect("failed to send data");
    let mut reader = BufReader::new(&stream); // create reader
    reader.read_to_end(&mut buf).unwrap();
    let ipc_msg_from_server: IpcMessageToClient = bincode::deserialize(&buf).unwrap();
    if let Some(error_serv) = ipc_msg_from_server.errors {
        let answer = format!("error from server {error_serv}");
        CString::new(answer).unwrap()
    } else if ipc_msg_from_server.turning.is_some() {
        let answer = format!(
            "device {} is now enabled! [{}]",
            ipc_msg_from_server.devname,
            ipc_msg_from_server.state.unwrap()
        );
        CString::new(answer).unwrap()
    } else {
        let answer = format!(
            "Retrieved info from the home server, devname: {}, state: {}, info: {}",
            ipc_msg_from_server.devname,
            ipc_msg_from_server.state.unwrap(),
            ipc_msg_from_server.property.unwrap()
        );
        CString::new(answer).unwrap()
    }
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
