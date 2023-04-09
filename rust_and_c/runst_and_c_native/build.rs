use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("library").display()
    );
    println!("cargo:rustc-link-lib={}", "testmy");
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}",
        Path::new(&dir).join("library").display()
    );
}
