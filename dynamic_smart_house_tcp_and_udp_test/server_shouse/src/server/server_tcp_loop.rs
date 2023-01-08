use lib_shouse::home::home::home::SmartHouse;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::io::{BufReader, BufWriter, Error};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

static SERIAL_MSG: AtomicUsize = AtomicUsize::new(0);

pub fn tcp_main_loop(listener: TcpListener, some_house: Arc<Mutex<SmartHouse>>) {
    let pattern = "message to client:".to_string();
    println!("launching server application");
    while let Some(stream) = listener.incoming().next() {
        if stream.is_err() {
            continue;
        }
        let stream = stream.unwrap();
        let mut writer = BufWriter::new(&stream);
        let mut reader = BufReader::new(&stream);
        let mut text_msg = String::new(); // for message
        if let Ok(data_readed) = read_all_data(&mut reader) {
            // process data
            let ipc_msg_from_client: IpcMessage = bincode::deserialize(&data_readed).unwrap();
            let dev_name = ipc_msg_from_client.devname;
            println!("looking for a device >>{dev_name}<<");
            //let mut room_name_found = String::new(); // oerkill?
            //let mut dev_name_found = String::new();
            let rom_dev: Option<(String, String)> = some_house
                .lock()
                .unwrap()
                .test_whether_a_dev_exists(&dev_name);
            let (room_name_found, dev_name_found) = if rom_dev.is_some() {
                // ???????
                println!("found valid dev! {dev_name}");
                rom_dev.unwrap()
            } else {
                // None
                println!("no such device!, aborting connection");
                text_msg.push_str(" no such device in the house! :");
                text_msg.push_str(&dev_name);
                send_all_data(&text_msg, &mut writer);
                continue;
            };
            match ipc_msg_from_client.state {
                IpcState::Get_state => std::fmt::write(
                    // send current data
                    &mut text_msg,
                    format_args!(
                        "{}, dev name: {}, property: {}, device is turned on: {}",
                        pattern,
                        dev_name,
                        some_house
                            .lock()
                            .unwrap()
                            .get_device_property(dev_name_found.as_str())
                            .unwrap(),
                        some_house
                            .lock()
                            .unwrap()
                            .get_device_state(dev_name_found.as_str())
                            .unwrap(),
                    ),
                )
                .expect("error writing message"),
                IpcState::Set_state => {} // do nothing
                IpcState::Turn_on => {
                    assert!(some_house
                        .lock()
                        .unwrap()
                        .change_dev_state_in_room(&room_name_found, &dev_name_found, true)
                        .is_ok());
                    std::fmt::write(
                        &mut text_msg,
                        format_args!(
                            "{}, dev name: {}, {}",
                            pattern, dev_name_found, "turned on the device!"
                        ),
                    )
                    .expect("error whiting message");
                }
                IpcState::Turn_off => {
                    assert!(some_house
                        .lock()
                        .unwrap()
                        .change_dev_state_in_room(&room_name_found, &dev_name_found, false)
                        .is_ok());
                    std::fmt::write(
                        &mut text_msg,
                        format_args!(
                            "{}, dev name: {}, {}",
                            pattern, dev_name_found, "turned off the device!"
                        ),
                    )
                    .expect("error whiting message");
                }
            }
            //println!("readed message: {:?}",String::from_utf8_lossy(&data_readed));
        } else {
            println!("error while reading data");
        }
        send_all_data(&text_msg, &mut writer);
        println!("closing connection!");
        SERIAL_MSG.fetch_add(1, Ordering::SeqCst);
    }
}
fn read_all_data(reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>, Error> {
    let received_data = reader.fill_buf()?.to_vec();
    reader.consume(received_data.len()); // mark as readed
    Ok(received_data)
}

fn send_all_data(data: &str, writer: &mut BufWriter<&TcpStream>) {
    let _ = writer.write_all(data.as_bytes());
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
