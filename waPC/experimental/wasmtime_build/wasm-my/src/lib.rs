use wapc_guest as wapc;

#[no_mangle]
pub fn wapc_init() {
    wapc::register_function("ints", ints);
}

fn ints(msg: i32) -> wapc::CallResult {
    wapc::console_log(&format!("performing magic"));
    let _res = wapc::host_call("binding", "sample:namespace", "int_call", msg)?;
    let _res = wapc::host_call("binding", "sample:namespace", "pong", msg)?;
    Ok(msg.to_vec())
}
