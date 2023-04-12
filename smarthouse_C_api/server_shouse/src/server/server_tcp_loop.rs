#![allow(unused_imports)]
use lib_shouse::home::home::home::SmartHouse;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::io::{BufReader, BufWriter, Error};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

static SERIAL_MSG: AtomicUsize = AtomicUsize::new(0);

pub fn tcp_main_loop(listener: TcpListener, some_house: Arc<Mutex<SmartHouse>>) {
    println!("launching server application");
    while let Some(stream) = listener.incoming().next() {
        if stream.is_err() {
            continue;
        }
        let stream = stream.unwrap();
        let mut writer = BufWriter::new(&stream);
        let mut reader = BufReader::new(&stream);
        if let Ok(data_readed) = read_all_data(&mut reader) {
            // process data
            let ipc_msg_from_client: IpcMessage = bincode::deserialize(&data_readed).unwrap();
            let dev_name = ipc_msg_from_client.devname;
            println!("looking for a device >>{dev_name}<<");
            let rom_dev: Option<(String, String)> = some_house
                .try_lock()
                .unwrap()
                .test_whether_a_dev_exists(&dev_name);
            let mut property = String::new();
            let (room_name_found, dev_name_found) = if rom_dev.is_some() {
                // ??????? COMPLICATED
                println!("found valid dev! {dev_name}");
                rom_dev.unwrap()
            } else {
                // None
                println!("no such device!, aborting connection");
                let error_msg = IpcMessageToClient::new_error(
                    &dev_name,
                    format!(">>{}<< no such device!", &dev_name).as_ref(),
                );
                let constructed_msg = bincode::serialize(&error_msg).unwrap();
                send_all_data(&constructed_msg, &mut writer);
                continue;
            };
            let mut ipc_msg_to_client: Vec<u8> = vec![];
            match ipc_msg_from_client.state {
                IpcState::Get_state => {
                    let dev_property = {
                        // complicated!
                        some_house
                            .try_lock()
                            .unwrap()
                            .get_device_property(dev_name_found.as_str())
                            .unwrap()
                    };
                    property.push_str(&dev_property);
                    let dev_state = {
                        some_house
                            .try_lock()
                            .unwrap()
                            .get_device_state(dev_name_found.as_str())
                            .unwrap()
                    };
                    ipc_msg_to_client.append(
                        &mut bincode::serialize(&IpcMessageToClient::new(
                            &dev_name_found,
                            &property,
                            dev_state,
                            &room_name_found,
                        ))
                        .unwrap(),
                    );
                }
                IpcState::Set_state => {
                    continue;
                } // do nothing
                IpcState::Turn_on => {
                    assert!(some_house
                        .try_lock()
                        .unwrap()
                        .change_dev_state_in_room(&room_name_found, &dev_name_found, true)
                        .is_ok());
                    ipc_msg_to_client.append(
                        &mut bincode::serialize(&IpcMessageToClient::new_turning(&dev_name, true))
                            .unwrap(),
                    );
                }
                IpcState::Turn_off => {
                    assert!(some_house
                        .lock()
                        .unwrap()
                        .change_dev_state_in_room(&room_name_found, &dev_name_found, false)
                        .is_ok());
                    ipc_msg_to_client.append(
                        &mut bincode::serialize(&IpcMessageToClient::new_turning(&dev_name, false))
                            .unwrap(),
                    );
                }
            }
            println!("dev {}", &dev_name_found);
            send_all_data(&ipc_msg_to_client, &mut writer);
            println!("closing connection!");
            SERIAL_MSG.fetch_add(1, Ordering::SeqCst);

            //println!("readed message: {:?}",String::from_utf8_lossy(&data_readed));
        } else {
            println!("error while reading data");
        }
    }
}
fn read_all_data(reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>, Error> {
    let received_data = reader.fill_buf()?.to_vec();
    reader.consume(received_data.len()); // mark as readed
    Ok(received_data)
}

fn send_all_data(data: &[u8], writer: &mut BufWriter<&TcpStream>) {
    let _ = writer.write_all(data);
    let _ = writer.flush();
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

impl IpcMessageToClient {
    fn new(devname: &str, property: &str, state: bool, room_name: &str) -> Self {
        Self {
            devname: devname.to_owned(),
            property: Some(property.to_owned()),
            state: Some(state),
            room_name: Some(room_name.to_owned()),
            errors: None,
            turning: None,
        }
    }
    fn new_error(devname: &str, error: &str) -> Self {
        Self {
            devname: devname.to_owned(),
            property: None,
            state: None,
            room_name: None,
            errors: Some(error.to_owned()),
            turning: None,
        }
    }
    fn new_turning(devname: &str, state: bool) -> Self {
        Self {
            devname: devname.to_owned(),
            property: None,
            state: Some(state),
            room_name: None,
            errors: None,
            turning: Some(()),
        }
    }
}
