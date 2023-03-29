use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use wapc::CallResult;
use wapc_guest as wapc;
#[no_mangle]
pub fn wapc_init() {
    wapc::register_function("ping", ping);
}
fn ping(msg: &[u8]) -> CallResult {
    // Note how this uses println!() directly vs the non-wasi sample which logs
    // via the host and console_log()
    println!(
        "IN_WASI: Received request for `ping` operation with payload : {}",
        std::str::from_utf8(msg).unwrap()
    );

    let mut hasher = DefaultHasher::new();
    123123.hash(&mut hasher);
    //let wrapped = WrapPayload::convert(msg)
    //let x = fxhash::hash32(wrapped);
    //let str1 = "REPLACED STRING".as_bytes();
    let str1 = hasher.finish();
    let str1 = &str1.to_be_bytes();
    let _res = wapc::host_call("binding", "sample:namespace", "pong", str1)?;
    Ok(str1.to_vec())
}
/*
struct WrapPayload<'a> {
    payload: &'a [u8],
}
impl<'a> WrapPayload<'a> {
    fn convert(pl: &[u8]) -> Self {
        Self { payload: pl }
    }
}

impl<'a> Hash for WrapPayload<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.payload.hash(state);
    }
}*/
