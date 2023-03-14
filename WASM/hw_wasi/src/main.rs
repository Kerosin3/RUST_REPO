use std::env;
use std::io::prelude::*;
use std::io::Write;
fn main() -> std::io::Result<()> {
    println!("starting!");
    let key = "HOME";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }
    let mut file = std::fs::File::create("/helloworld/hw.txt").unwrap();
    file.write(b"heheh").unwrap();
    Ok(())
}
