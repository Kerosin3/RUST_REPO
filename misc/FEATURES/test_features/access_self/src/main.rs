fn main() {
    let mut x = SomeStruct::new();
    SomeStruct::assign(&mut x, 15);
    println!("value now is {:?}", x);
    println!("------------------------");
    let mut xy = SomeStruct::new();
    xy.do_something();
    println!("value now is {:?}", xy);
}
//#------------------------simple-------------------------
#[derive(Debug)]
struct SomeStruct {
    field1: i32,
}

impl SomeStruct {
    fn new() -> Self {
        Self { field1: 0 }
    }
    fn assign(&mut self, arg1: i32) {
        self.field1 = arg1;
    }
}
//-----------------------------------------------------------
trait SomeTrait {
    type Out;
    fn do_something(&mut self) -> Self::Out;
}

impl SomeTrait for SomeStruct {
    type Out = Self;
    fn do_something(&mut self) -> Self::Out {
        let out = SomeStruct::new();
        SomeStruct::assign(self, 5); // <----
        out
    }
}
