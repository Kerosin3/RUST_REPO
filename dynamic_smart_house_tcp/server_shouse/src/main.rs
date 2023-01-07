use lib_shouse::home::home::home::*;
use serde::{Deserialize, Serialize};
use server_shouse::server_data::server_data::*;
use std::cell::RefCell;
use std::io::{BufRead, Write};
use std::io::{BufReader, BufWriter, Error};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;

static mut SERIAL: usize = 0;

fn main() {
    let mut some_house = SmartHouse::new();
    let room_0 = "room_0".to_string();
    some_house.append_room(&room_0).unwrap();
    let dev0: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(SmartSocket::new()));
    let _dev0_handler = some_house.append_dev_to_a_room(&room_0, &dev0).unwrap(); // append dev to room0
    _dev0_handler.property_change_state(9000_f32).unwrap();
    //println!("dev name is {}", _dev0_handler.get_devname().unwrap());
    //println!("current property state: {}", _dev0_handler.get_property_state().unwrap() );
    //------------------------------------------
    let listener = TcpListener::bind("127.0.0.1:12345").expect("bind failed");
    //listener.set_nonblocking(true).expect("error setting non blocking"); hogh workload
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
            println!("looking for a device >>{}<<", dev_name);
            if some_house.test_whether_a_dev_exists(&dev_name).is_none() {
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
                        _dev0_handler.get_devname().unwrap(),
                        _dev0_handler.get_property_state().unwrap(),
                        _dev0_handler.get_state().unwrap()
                    ),
                )
                .expect("error writing message"),
                IpcState::Set_state => {} // do nothing
                IpcState::Turn_on => {
                    assert!(_dev0_handler.change_state(true).is_ok());
                    std::fmt::write(
                        &mut text_msg,
                        format_args!(
                            "{}, dev name: {}, {}",
                            pattern,
                            _dev0_handler.get_devname().unwrap(),
                            "turned on the device!"
                        ),
                    )
                    .expect("error whiting message");
                }
                IpcState::Turn_off => {
                    assert!(_dev0_handler.change_state(false).is_ok());
                    std::fmt::write(
                        &mut text_msg,
                        format_args!(
                            "{}, dev name: {}, {}",
                            pattern,
                            _dev0_handler.get_devname().unwrap(),
                            "turned off the device!"
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
        println!(
            "current property state: {}, enabled? : {}",
            _dev0_handler.get_property_state().unwrap(),
            _dev0_handler.get_state().unwrap()
        );

        println!("closing connection!");
        unsafe {
            SERIAL += 1;
        }
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
