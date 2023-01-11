use thiserror::Error;
use anyhow::{Result, anyhow};
use std::io::{self, ErrorKind};

pub fn prog2_run(){
    println!("aaaaaaaaaaaaaa");
}

pub fn app2_imitator(arg:i32) -> Result<(),App2Error>{
    match arg {
        1 => Err(App2Error::App2Error1),
        2 => Err(App2Error::App2Error2),
        3 => Err(App2Error::App2CriticalError((  anyhow!("critical error: {}", arg)))),
        _ => Err(App2Error::NotCovered(( anyhow!("not covered error ")  )))
    }
}

#[derive(Debug, Error)]
//#[error(from_source)]
pub enum App2Error{
    #[error("App2Error1")]
    App2Error1,
    #[error("App2Error2")]
    App2Error2,
    #[error(transparent)]
    App2CriticalError(#[from] anyhow::Error),
    #[error(transparent)]
    NotCovered(anyhow::Error),

}
