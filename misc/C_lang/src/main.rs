fn main() {
    unsafe{
        print_num(55);
    }
}
extern "C" {
    fn print_num(num: i32);
}