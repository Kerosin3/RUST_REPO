use thiserror::Error;
use anyhow::{Result, anyhow};
use std::io::{self, ErrorKind};

pub fn err_from_lib_emulator(arg:i32)-> Result<(),LibErrors>{
   match arg {
       1 => Err(LibErrors::LibEror0(std::io::Error::new(ErrorKind::NotFound, "not found"))),
       2 => Err(LibErrors::LibEror2(( anyhow!("error! arg is {}",arg) ))),
       3 => Err(LibErrors::LibEror1(std::io::Error::new(ErrorKind::NotConnected, "not connected"))),
       _ => Ok(())
   } 
}

#[derive(Debug, Error)]
pub enum LibErrors{
    #[error("Io error {0}")]
    LibEror0(#[source] io::Error), // from IO
    #[error("Io error {0}")]
    LibEror1(#[source] io::Error), // from IO
    #[error(transparent)]
    LibEror2(#[from] anyhow::Error),
    #[error("LibError3")]
    LibErro3
}
