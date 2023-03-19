use log::{debug, error, info, log_enabled, Level};
use std::env;
use std::io::{self, Read, Write};
fn main() -> io::Result<()> {
    println!("all env keys:");
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
    std::env::var("RUST_LOG").expect("RUST_LOG environment variable unset");

    env_logger::init();
    debug!("this is a debug {}", "message");
    info!("this is a info {}", "message");
    error!("this is printed by default");
    Ok(())
}
