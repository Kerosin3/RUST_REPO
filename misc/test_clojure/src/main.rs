use std::thread;
use std::ops::Add;

fn main() {
    let a = 42;
    let b = &a;
//  std::mem::drop(a);
    let x = b.add(5);
    println!("value is {}",b);
    println!("value is {}",x);
}
