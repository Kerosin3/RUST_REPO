use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::io::Write;
use std::{io::Read, time::Instant};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use wapc_codec::messagepack::{deserialize, serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct PersonSend {
    first_name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct PersonHashedRecv {
    first_name: String,
    hash: u64,
}

use wapc::WapcHost;
use wasm3_runner::Wasm3EngineProvider;

pub fn main() -> Result<(), wapc::errors::Error> {
    env_logger::init();
    //---
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "host excutable has started!").expect("cannot write to stdout");
    //-- terminal
    let n = Instant::now();
    let file = &std::env::args()
        .nth(1)
        .expect("WASM file should be passed as the first CLI parameter");
    let func = &std::env::args()
        .nth(2)
        .expect("waPC guest function to call should be passed as the second CLI parameter");
    let name = &std::env::args().nth(3).expect("pass a name");
    let name = name.to_string();
    let module_bytes = std::fs::read(file).expect("WASM could not be read");
    let engine = Wasm3EngineProvider::new(&module_bytes);

    let host = WapcHost::new(Box::new(engine), Some(Box::new(host_callback)))?;

    println!("Calling guest (wasm) function '{}'", func);
    // supply person
    let person = PersonSend { first_name: name };
    let bytes3: SmallVec<[u8; 1024]> = serialize(&person).unwrap().into();
    let encoded = hex::encode(bytes3.clone());
    println!("serialized message: {}", encoded);
    println!("calling wasm guest");
    let res = host.call(func, &bytes3)?;
    let round_trip: PersonHashedRecv = deserialize(&res).unwrap();
    println!("getting response from guest");
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
    println!(
        "Guest {} invoked '{}->{}:{}' on the host with a payload of '{}'",
        id,
        bd,
        ns,
        op,
        hex::encode(payload)
    );
    Ok(vec![])
}
