fn main(){
    let x = Point::new();
    dynamic(&x);
    static_dispatch(x);
    let z = 13;
    static_dispatch(z);

}
struct Point{
    x: i32,
}
impl Point {
    fn new() -> Self{
        Point{x:5}
    }
}

fn static_dispatch<S: SomeTrait>(s:S){
    s.gen();
}
impl SomeTrait for i32{
    type Out = (i32);

    fn gen(&self) -> i32 {
       42_i32
    }
}

fn dynamic(arg1: &dyn SomeTrait<Out=(i32)>) -> i32{
    arg1.gen()
}

impl SomeTrait for Point{
    type Out = i32;
    fn gen(&self) -> Self::Out{
        self.x * self.x
    }
}

trait SomeTrait{
    type Out;
    fn gen(&self) -> i32;
}

/*
fn main() {
    let x: Box<dyn AsRef<Str>>= Box::new(String::from("dasdasdasda"));
    let p_s: Box<dyn AsRef<str>> = x;
    println!("value is {}",strlen_dyn(p_s));
}
fn process(s: impl AsRef<str>) {
//fn process<T: AsRef<str>>(s:T){
    for i in s.as_ref().chars() {
        println!("{i}");
    }
}
fn strlen_dyn(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}
*/
