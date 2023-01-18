use anyhow::{anyhow, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use thiserror::Error;
//use server_shouse::udp_termo_data_server::udp_termo_server;
//use server_shouse::update_termometer_client::run_termo_quering;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::io::AsyncWrite;
use tokio::io::BufReader;
use tokio::io::BufStream;
use tokio::io::{AsyncBufReadExt, BufWriter};
use tokio::io::{AsyncRead, ReadHalf, WriteHalf};
//use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
mod ipc_message;
use ipc_message::*;
mod server_socket_struct;
mod server_termometer_struct;
use lib_shouse::home::home::home::*;
use server_socket_struct::*;
use server_termometer_struct::*;
type customtype = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut some_house = SmartHouse::new();
    let room_0 = "room_0".to_string();
    some_house.append_room(&room_0).unwrap();
    let dev0 = wrap_device(SmartSocket::new());
    let dev1 = wrap_device(Termometer::new());
    let _dev0_handler = some_house.append_dev_to_a_room(&room_0, &dev0).unwrap(); // append dev0 to room0
    let _dev1_handler = some_house.append_dev_to_a_room(&room_0, &dev1).unwrap(); // append dev1 to room0
    _dev0_handler.property_change_state(9000_f32).unwrap();
    _dev1_handler.property_change_state(36.6_f32).unwrap();
    //-----------------event_loop-------------------
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
    let wrap_home = Arc::new(Mutex::new(some_house));
    loop {
        println!("loop cycle");
        let (socket, _) = listener.accept().await.unwrap();
        let sh = Arc::clone(&wrap_home);
        tokio::spawn(async move { handle_connection(socket, sh).await });
    }
    Ok(())
}

async fn handle_connection(
    mut socket: TcpStream,
    sm_obj: Arc<Mutex<SmartHouse>>,
) -> anyhow::Result<()> {
    let mut rw_stream = BufStream::new(socket);
    let mut buf_read = Vec::<u8>::new();

    loop {
        if let Ok(n) = rw_stream.fil(&mut buf_read).await {
            let msg_from_client: Message = Message::deserialize_message(&buf_read);
            //--------------------------process message here-----------------------------------
            let dev_name = msg_from_client.devname;
            let room_dev = match sm_obj.try_lock() {
                Err(_) => anyhow::bail!(HandleError::MutexError((anyhow!("error locking mutex")))),
                Ok(guard) => guard.test_whether_a_dev_exists(&dev_name),
            };
            let mut message_to_client = Message::new(NetMsgType::SendServer);
            message_to_client.assign_devname(dev_name.to_owned());
            message_to_client.assign_command(Command::MsgBack);
            let (room_name_found, dev_name_found) = if room_dev.is_some() {
                println!("found valid dev! {}", &dev_name);
                room_dev.unwrap()
            } else {
                todo!();
            };
            let mut info_property = String::new();
            match msg_from_client.command.unwrap() {
                Command::TurnOn => {
                    modify_house(
                        &sm_obj,
                        Command::TurnOn,
                        (&room_name_found, &dev_name_found),
                    )
                    .await?; //works?
                }
                Command::TurnOff => {
                    modify_house(
                        &sm_obj,
                        Command::TurnOff,
                        (&room_name_found, &dev_name_found),
                    )
                    .await?; //works?
                }
                Command::GetProperty => {
                    info_property = modify_house(
                        // shadows??
                        &sm_obj,
                        Command::GetProperty,
                        (&room_name_found, &dev_name_found),
                    )
                    .await?;
                    message_to_client.assign_info(info_property); //assign msg
                }
                Command::MsgBack => todo!(),
            }
            rw_stream.write(b"11111111111111111111\r\n").await?;
            println!("going to send");
            rw_stream.flush().await?;

            println!("sended!");
            //rw_stream.write_all(&message_to_client.serialize_message());
        } else {
            anyhow::bail!(HandleError::SockError) // cannot read from socket
        }
    }
}

async fn modify_house(
    sm_obj: &Arc<Mutex<SmartHouse>>,
    cmd: Command,
    (room, dev): (&str, &str),
) -> anyhow::Result<String> {
    match cmd {
        Command::TurnOn => {
            if let Ok(mut guard) = sm_obj.try_lock() {
                guard.change_dev_state_in_room(room, dev, true);
                Ok(String::new())
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::TurnOff => {
            if let Ok(mut guard) = sm_obj.try_lock() {
                guard.change_dev_state_in_room(room, dev, false);
                Ok(String::new())
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::GetProperty => {
            if let Ok(mut guard) = sm_obj.try_lock() {
                Ok(guard.get_device_property(dev)?) // HOW?????
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::MsgBack => todo!(),
    }
}

fn wrap_device<T: 'static + lib_shouse::home::home::home::Device + Send + Sync>(
    some_device: T,
) -> Arc<Mutex<dyn Device + Send>> {
    Arc::new(Mutex::new(some_device))
}

#[derive(Debug, Error)]
pub enum HandleError {
    #[error("Error locking smart house object")]
    ErrorHouseLocking,
    #[error("Error reading from socket")]
    SockError,
    #[error(transparent)]
    tokio_error(#[from] anyhow::Error),
    //#[error(transparent)]
    //mutex_error(#[from] std::sync::TryLockError<T>),
    #[error(transparent)]
    MutexError(anyhow::Error),
}

/*
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
}*/
