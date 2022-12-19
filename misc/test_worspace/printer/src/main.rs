#[cfg(feature = "feature1")]
fn main() {
    println!("Hello, world!");
    adder::add(54, 123);
    if cfg!(target_os="linux"){
        println!("compiling for linux");
    }

    if cfg!(feature = "feature1"){
        println!("compiling with feature1");
    } else {
        println!("compiling with no features");
    }
}

