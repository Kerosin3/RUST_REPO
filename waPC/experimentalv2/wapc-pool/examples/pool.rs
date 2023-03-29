use std::fs::read;

use wapc::WapcHost;
use wapc_codec::messagepack::{deserialize, serialize};
use wapc_pool::HostPoolBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let buf = read("wapc_guest_test.wasm").expect("cannot find wasm file");

    let engine = wasmtime_runner::WasmtimeEngineProviderBuilder::new()
        .module_bytes(&buf)
        .build()?;

    let pool = HostPoolBuilder::new()
        .name("pool example")
        .factory(move || {
            let engine = engine.clone();
            WapcHost::new(Box::new(engine), None).unwrap()
        })
        .max_threads(5)
        .build();

    let bytes = pool.call("echo", serialize("Hello!")?).await?;

    let result: String = deserialize(&bytes)?;

    println!("Wasm module returned: {}", result);

    Ok(())
}
