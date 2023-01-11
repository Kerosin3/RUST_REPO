use thiserror::Error;
use anyhow::{Result, anyhow};
use std::io::{self, ErrorKind};

mod prog1;
mod prog2;
mod some_lib;

use prog1::{*};
use prog2::{*};
use some_lib::{*};

fn main() {
    for i in 0..5 {
        println!("{}",call_prog(i).unwrap_err());
    }
}

fn call_prog(arg:i32) -> Result<(), anyhow::Error>{
    app1_imitator(arg)?;
    app2_imitator(arg)?;
    err_from_lib_emulator(arg)?;
    Ok(())
}
