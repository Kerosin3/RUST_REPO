use std::ops::Add;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
fn main() {
    println!("name is: {}", HELLO_WORLD);
    unsafe {
        for i in 0..5{
            add_count(&mut COUNTER);
            println!("value is {:?}",COUNTER);
        }
    }
}

fn add_count(var: &mut u32 ){
    unsafe{
        var.add(1);
    }
}