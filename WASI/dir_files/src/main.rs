use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("trying to create and write file");
    let mut file1 = File::create("/foo1/fooa.txt")?;
    let mut file2 = File::create("/foo2/foob.txt")?;
    file1.write_all(b"Hello, world 1!")?;
    file2.write_all(b"Hello, world 2!")?;
    let mut file = File::open("/foo1/readfromme.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("readed content is {:?}", contents);
    Ok(())
}
