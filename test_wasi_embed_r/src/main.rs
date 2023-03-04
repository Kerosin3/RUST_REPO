use anyhow::Result;
use std::borrow::{Borrow, BorrowMut};
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

struct MyState {
    message: String,
    wasi: WasiCtx,
}

fn main() -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(
        &engine,
        MyState {
            message: format!("hello!"),
            wasi,
        },
    );

    // ...

    Ok(())
}
