use thiserror::Error;
use anyhow::{Result, anyhow};
use std::io::{self, ErrorKind};

pub fn prog1_run(){
    println!("aaaaaaaaaaaaaa");
}

pub fn app1_imitator(arg:i32) -> Result<(),App1Error>{
    match arg {
        1 => Err(App1Error::App1Error1),
        2 => Err(App1Error::App1Error2),
        3 => Err(App1Error::UnexpectedAppError((  anyhow!("Missing attribute: {}", arg)))),
        _ => Err(App1Error::NotCovered(( anyhow!("not covered error ")  )))
    }
}

#[derive(Debug, Error)]
//#[error(from_source)]
pub enum App1Error{
    #[error("App1Error1")]
    App1Error1,
    #[error("App1Error2")]
    App1Error2,
    #[error(transparent)]
    UnexpectedAppError(#[from] anyhow::Error),
    #[error(transparent)]
    NotCovered(anyhow::Error),

}
