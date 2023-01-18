use anyhow::{anyhow, Result};
use bytes::Bytes;
use thiserror::Error;
//use server_shouse::udp_termo_data_server::udp_termo_server;
//use server_shouse::update_termometer_client::run_termo_quering;
//use socket2::{Domain, Protocol, SockRef, Socket, Type};
use futures::SinkExt;
use futures_util::*;
use socket2::SockRef;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::io::BufStream;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
//use tokio::sync::Mutex;
mod ipc_message;
use ipc_message::*;
mod server_socket_struct;
mod server_termometer_struct;
use lib_shouse::home::home::home::*;
use server_socket_struct::*;
use server_termometer_struct::*;
//type customtype = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
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
    let wrap_home = Arc::new(Mutex::new(some_house));
    //tokio::spawn(async move { handle_connection(socket, sh).await });
    let mut handlers = vec![];
    if let Ok(tcp_listener) = TcpListener::bind("127.0.0.1:12345").await {
        while let Ok((tcp_stream, _socket_addr)) = tcp_listener.accept().await {
            let sh = Arc::clone(&wrap_home);
            let handle = tokio::spawn(async move {
                // -----------------------------SPAWN TASK--------------------------------------
                server_event_loop(tcp_stream, sh).await
            });
            handlers.push(handle);
        }
    } else {
        println!("error server binding");
    }
    for h in handlers {
        // not working
        match h.await {
            Ok(_) => println!("task finished ok!"),
            Err(e) => println!("task finished with error {:?}", e),
        }
    }
    Ok(())
}
async fn server_event_loop(
    tcp_stream: TcpStream,
    sm_obj: Arc<Mutex<SmartHouse>>,
) -> anyhow::Result<()> {
    let socket_ref = SockRef::from(&tcp_stream);
    socket_ref.set_nonblocking(true)?;
    socket_ref.set_nodelay(true).unwrap();
    socket_ref.listen(128); // panics on unwrap
    socket_ref.set_reuse_port(true)?;
    socket_ref.set_reuse_address(true)?;
    socket_ref.set_recv_buffer_size(2048)?;
    socket_ref.set_send_buffer_size(2048)?;
    std::mem::drop(socket_ref); // ok
    let codec = LengthDelimitedCodec::builder()
        .length_field_offset(0) // default value
        .length_field_type::<u16>()
        .length_adjustment(0) // default value
        .new_codec();
    let stream_buf = BufStream::new(tcp_stream);
    let mut framed_stream = Framed::new(stream_buf, codec);
    let frame = Bytes::from("hello from server, what do you want?");
    framed_stream.send(frame).await.unwrap();
    // INFINITE LOOP!
    let mut i = 0_usize;
    '_accept_client_request: while let Some(frame) = framed_stream.next().await {
        match frame {
            Ok(f) => {
                if i == 0 {
                    println!("readed 0 frame: {:?}", f);
                    if String::from_utf8_lossy(&f.to_vec()) == "client magic words!" {
                        let frame = Bytes::from("ASK");
                        framed_stream.send(frame).await?;
                        println!("server sent ask");
                    } else {
                        println!("wrong answer");
                        anyhow::bail!(HandleError::KeyError) // wrong answer
                    }
                    i += 1;
                } else {
                    //--------------------------------------------------------------------//
                    //PROCESS INFO MESSAGE!
                    println!("readed frame: {:?}", f);
                    i += 1;
                    let msg_from_client: Message = Message::deserialize_message(&f.to_vec());
                    let dev_name = msg_from_client.devname;
                    let room_dev = match sm_obj.try_lock() {
                        Err(_) => {
                            anyhow::bail!(HandleError::MutexError(anyhow!("error locking mutex")))
                        }
                        Ok(guard) => guard.test_whether_a_dev_exists(&dev_name),
                    };
                    let mut message_to_client = Message::new(NetMsgType::SendServer);
                    message_to_client.assign_devname(dev_name.to_owned());
                    message_to_client.assign_command(Command::MsgBack);
                    let (room_name_found, dev_name_found) = if room_dev.is_some() {
                        println!("found valid dev! {}", &dev_name);
                        room_dev.unwrap()
                    } else {
                        let msg_back = format!("devname:{} not found", &dev_name);
                        framed_stream.send(Bytes::from(msg_back)).await?;
                        anyhow::bail!(HandleError::NoSuchDeviceExists)
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
                            message_to_client.assign_info(info_property.to_owned());
                            //assign msg
                        }
                        Command::MsgBack => todo!(),
                    }
                    // write answer
                    let frame = Bytes::from(info_property);
                    framed_stream.send(frame).await.unwrap();
                    return Ok(()); //run once
                }
            }
            Err(_) => {
                anyhow::bail!(HandleError::FrameError) // cannot read from socket
            }
        }
    }
    return anyhow::bail!(HandleError::WrongSeq); // }
}

fn wrap_device<T: 'static + lib_shouse::home::home::home::Device + Send + Sync>(
    some_device: T,
) -> Arc<Mutex<dyn Device + Send>> {
    Arc::new(Mutex::new(some_device))
}

async fn modify_house(
    sm_obj: &Arc<Mutex<SmartHouse>>,
    cmd: Command,
    (room, dev): (&str, &str),
) -> anyhow::Result<String> {
    match cmd {
        Command::TurnOn => {
            if let Ok(mut guard) = sm_obj.try_lock() {
                guard.change_dev_state_in_room(room, dev, true)?;
                Ok(String::new())
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::TurnOff => {
            if let Ok(mut guard) = sm_obj.try_lock() {
                guard.change_dev_state_in_room(room, dev, false)?;
                Ok(String::new())
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::GetProperty => {
            if let Ok(guard) = sm_obj.try_lock() {
                Ok(guard.get_device_property(dev)?) // HOW?????
            } else {
                anyhow::bail!(HandleError::ErrorHouseLocking)
            }
        }
        Command::MsgBack => todo!(),
    }
}

#[derive(Debug, Error)]
pub enum HandleError {
    #[error("Error locking smart house object")]
    ErrorHouseLocking,
    #[error("Error reading from socket")]
    SockError,
    #[error("Error keyprase")]
    KeyError,
    #[error("No such device")]
    NoSuchDeviceExists,
    #[error("Wrong sequence")]
    WrongSeq,
    #[error("frame processing error")]
    FrameError,
    #[error("frame semding error")]
    FrameErrorSend,
    #[error(transparent)]
    TokioError(#[from] anyhow::Error),
    //#[error(transparent)]
    //mutex_error(#[from] std::sync::TryLockError<T>),
    #[error(transparent)]
    MutexError(anyhow::Error),
}
