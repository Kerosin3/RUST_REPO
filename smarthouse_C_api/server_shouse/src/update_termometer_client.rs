use lib_shouse::home::home::home::Device_Handler;
use std::net::UdpSocket;
use std::{thread, time};

pub fn run_termo_quering(handle: Device_Handler) {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to server");
    let mut buf = [0_u8; 4];
    loop {
        socket.send_to(b"1", "0.0.0.0:20001").unwrap();
        let _size = socket.recv(&mut buf).unwrap();
        handle
            .property_change_state(f32::from_be_bytes(buf))
            .unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}
