use serde::{Deserialize, Serialize};

use wapc_codec::messagepack::{deserialize, serialize};
use wapc_guest as wapc;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

#[no_mangle]
pub fn wapc_init() {
    wapc::register_function("ping", ping);
}

fn ping(msg: &[u8]) -> wapc::CallResult {
    //    wapc::console_log(&format!(
    //        "IN_WASM: Received request for `ping` operation with payload : {}",
    //        std::str::from_utf8(msg).unwrap()
    //   ));

    let mut round_trip: Person = deserialize(&msg)?;
    let name = "notSamuel".to_owned();
    round_trip.first_name = name;
    let bytes = serialize(&round_trip)?;
    let _res = wapc::host_call("binding", "sample:namespace", "pong", &bytes)?;
    Ok(bytes.to_vec())
}
/*
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person = Person {
        first_name: "Samuel".to_owned(),
        last_name: "Clemens".to_owned(),
        age: 49,
    };

    println!("Original : {:?}", person);

    let bytes = serialize(&person)?;

    println!("Serialized messagepack bytes: {:?}", bytes);

    let round_trip: Person = deserialize(&bytes)?;

    println!("Deserialized : {:?}", round_trip);

    Ok(())
}*/
