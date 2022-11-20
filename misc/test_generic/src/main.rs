fn main() {
    let x = 10;
    let y = 66;
    println!("result is {}", foo(x,y) );
}

fn foo<T: std::ops::Add + Copy>(arg1:T,arg2:T) -> T::Output {
    arg1 + arg2
}
