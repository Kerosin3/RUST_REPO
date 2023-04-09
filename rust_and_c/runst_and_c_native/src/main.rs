#[link(name = "testmy", kind = "dylib")]
extern "C" {
    fn ret_num() -> i32;
}
fn main() {
    println!("Hello, world! {}", unsafe { ret_num() });
}
