use std::error::Error;

use std::env;
use std::path::Path;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("val: {}", call_dynamic()?);
    Ok(())
}
fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir).join("library").join("libtestmy.so");
    unsafe {
        let lib = libloading::Library::new(path)?;
        let func: libloading::Symbol<unsafe extern "C" fn() -> u32> = lib.get(b"ret_num")?;
        Ok(func())
    }
}
