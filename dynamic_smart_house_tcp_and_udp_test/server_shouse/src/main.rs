#![allow(dead_code)]
#![allow(clippy::new_without_default)]
#![feature(mutex_unlock)]
#![allow(unused_imports)]
use lib_shouse::home::home::home::*;
use server_shouse::server::server_socket_struct::*;
use server_shouse::server::server_tcp_loop::*;
use server_shouse::server::server_termometer_struct::*;
use std::cell::RefCell;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

static mut SERIAL: usize = 0;

fn main() {
    let mut some_house = SmartHouse::new();
    let room_0 = "room_0".to_string();
    some_house.append_room(&room_0).unwrap();
    let dev0 = wrap_device(SmartSocket::new());
    let dev1 = wrap_device(Termometer::new());
    let _dev0_handler = some_house.append_dev_to_a_room(&room_0, &dev0).unwrap(); // append dev0 to room0
    let _dev1_handler = some_house.append_dev_to_a_room(&room_0, &dev1).unwrap(); // append dev1 to room0
    _dev0_handler.property_change_state(9000_f32).unwrap();
    _dev1_handler.property_change_state(36.6_f32).unwrap();
    //------------------------------------------
    let listener = TcpListener::bind("127.0.0.1:12345").expect("bind failed");
    //listener.set_nonblocking(true).expect("error setting non blocking"); hogh workload

    // start tcp loop
    let wrap_home = Arc::new(Mutex::new(some_house));
    let tcp_thread = Arc::clone(&wrap_home);
    let tcp_loop_thrd = thread::spawn(move || tcp_main_loop(listener, tcp_thread));
    //tcp_main_loop(listener, some_house);
    tcp_loop_thrd.join().unwrap();
}

fn wrap_device<T: 'static + lib_shouse::home::home::home::Device + Send + Sync>(
    some_device: T,
) -> Arc<Mutex<dyn Device + Send>> {
    Arc::new(Mutex::new(some_device))
}
