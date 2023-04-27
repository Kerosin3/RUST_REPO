use postcard::{de_flavors::Slice, Deserializer};
use postcard::{ser_flavors, serialize_with_flavor};
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::{Bytes, BytesOrString, NoneAsEmptyString};
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::ops::Deref;

fn main() -> Result<(), std::io::Error> {
    let cur_dir_path = env::current_dir()?; // get current dir
    let filename = cur_dir_path.join("somefile");
    {
        let mut opened_file = File::create(&filename)?;
        let data = [1_u8, 2_u8];
        let mut struct_to_ser = Arrays::new();
        let mut msg = "helloo".to_string();
        for _i in 0..10 {
            struct_to_ser.assign_data(&msg);
            println!("struct is {:?}", struct_to_ser);
            let size = serialize_with_flavor(&struct_to_ser, ser_flavors::Size::default()).unwrap();
            println!("size is {}", size);
            let t: Vec<u8> = postcard::to_allocvec(&struct_to_ser).unwrap();
            opened_file.write_all(&t)?;
            struct_to_ser.incrementserial();
        }
    }
    let mut open_file = File::open(&filename)?;
    let mut buffer_read: Vec<u8> = vec![];
    let _readed = open_file.read_to_end(&mut buffer_read)?;
    for _i in 0..10 {
        let recovered: Arrays = postcard::from_bytes(&buffer_read).unwrap();
        println!("{:?}", recovered);
    }
    Ok(())
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
struct Arrays<'a> {
    bytes: &'a [u8],
    serial: i32,
    message: &'a str,
}
impl<'a> Arrays<'a> {
    fn new() -> Self {
        Arrays::default()
    }
    fn assign_data(&mut self, msg: &'a str) {
        self.message = msg;
    }
    fn incrementserial(&mut self) {
        self.serial += 1;
    }
}
