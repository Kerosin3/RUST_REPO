use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::{io::Read, time::Instant};
use wapc_codec::messagepack::{deserialize, serialize};
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

use wapc::WapcHost;
use wasm3_rpc_example::Wasm3EngineProvider;

pub fn main() -> Result<(), wapc::errors::Error> {
    env_logger::init();
    let n = Instant::now();
    let file = &std::env::args()
        .nth(1)
        .expect("WASM file should be passed as the first CLI parameter");
    let func = &std::env::args()
        .nth(2)
        .expect("waPC guest function to call should be passed as the second CLI parameter");
    let payload = &std::env::args()
        .nth(3)
        .expect("The string payload to send should be passed as the third CLI parameter");

    let module_bytes = std::fs::read(file).expect("WASM could not be read");
    let engine = Wasm3EngineProvider::new(&module_bytes);

    let host = WapcHost::new(Box::new(engine), Some(Box::new(host_callback)))?;

    println!("Calling guest (wasm) function '{}'", func);
    // supply person
    let person = Person {
        first_name: "Samuel".to_owned(),
        last_name: "Clemens".to_owned(),
        age: 49,
    };
    let bytes3: SmallVec<[u8; 1024]> = serialize(&person).unwrap().into();
    println!("serialized: {:?}", bytes3);
    let res = host.call(func, &bytes3)?;
    let round_trip: Person = deserialize(&res).unwrap();

    println!("Deserialized : {:?}", round_trip);

    Ok(())
}

fn host_callback(
    id: u64,
    bd: &str,
    ns: &str,
    op: &str,
    payload: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    /* println!(
        "Guest {} invoked '{}->{}:{}' on the host with a payload of '{}'",
        id,
        bd,
        ns,
        op,
        ::std::str::from_utf8(payload).unwrap()
    );*/
    Ok(vec![])
}
