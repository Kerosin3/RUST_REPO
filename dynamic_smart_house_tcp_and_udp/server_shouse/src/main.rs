#![allow(dead_code)]
#![allow(clippy::new_without_default)]
use lib_shouse::home::home::home::*;
use server_shouse::server::server_socket_struct::*;
use server_shouse::server::server_tcp_loop::*;
use server_shouse::server::server_termometer_struct::*;
use std::cell::RefCell;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;

static mut SERIAL: usize = 0;

fn main() {
    let mut some_house = SmartHouse::new();
    let room_0 = "room_0".to_string();
    some_house.append_room(&room_0).unwrap();
    let dev0: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(SmartSocket::new()));
    let dev1: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(Termometer::new()));
    let _dev0_handler = some_house.append_dev_to_a_room(&room_0, &dev0).unwrap(); // append dev0 to room0
    let _dev1_handler = some_house.append_dev_to_a_room(&room_0, &dev1).unwrap(); // append dev1 to room0
    _dev0_handler.property_change_state(9000_f32).unwrap();
    _dev1_handler.property_change_state(36.6_f32).unwrap();
    //println!("dev name is {}", _dev0_handler.get_devname().unwrap());
    //println!("current property state: {}", _dev0_handler.get_property_state().unwrap() );
    //------------------------------------------
    let listener = TcpListener::bind("127.0.0.1:12345").expect("bind failed");
    //listener.set_nonblocking(true).expect("error setting non blocking"); hogh workload

    // start tcp loop
    //let mut  wrap_home = Arc::new(RwLock::new(some_house));
    //let mut tcp_thread = Arc::clone(&wrap_home);
    //let tcp_loop_thrd = thread::spawn(move ||tcp_main_loop(listener, tcp_thread));
    tcp_main_loop(listener, some_house);
}
