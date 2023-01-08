use rand::Rng;
use std::net::UdpSocket;
use std::{thread, time};

pub fn udp_termo_server() {
    let socket = UdpSocket::bind("0.0.0.0:20001");
    if socket.is_err() {
        println!("error binding to socket");
        std::process::exit(1);
    }
    let socket = socket.unwrap();
    let mut buf = [0_u8; 4];
    let mut rng = rand::thread_rng();
    loop {
        let (_sizes, sender) = socket.recv_from(&mut buf).unwrap();
        thread::sleep(time::Duration::from_secs(1));
        let num_to_send = rng.gen_range(10.0..50.0) as f32;
        //        println!("temp is {}",num_to_send);
        socket
            .send_to(&num_to_send.to_be_bytes(), sender)
            .expect("error while sending echo");
    }
}
