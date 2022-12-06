use std::io::{Error, ErrorKind};
fn main() {
    let x = SomeEnum::new(5).expect("some error");
    test_enum(&x).expect("error expect!");
}

pub enum SomeEnum{
    One(i32),
    Two(String),
    Tree(u64),
}

impl SomeEnum{
    fn decide(&self)->Result<i32,Error>{
        match &self {
            SomeEnum::One(val) => {
                println!("one enum");
                Ok(1)
            },
            SomeEnum::Two(val) => {
                println!("two enum");
                Ok(2)
            },
            SomeEnum::Tree(val) => {
                println!("tree enum");
                Ok(3)
            },
            _ => {
                Err(Error::new(ErrorKind::Other,"some error"))
            }
        }
    }
    fn new(choose:i32)->Result<Self,Error>{
        match choose {
            1 => Ok(SomeEnum::One(1_i32)),
            2 => Ok(SomeEnum::Two(String::from("two string"))),
            3 => Ok(SomeEnum::Tree(50_u64)),
            _ => Err(Error::new(ErrorKind::Other,"some error"))
        }
    }
}

fn test_enum(arg1:&SomeEnum) -> Result<i32,Error>{
    match arg1.decide()? {
        1 => Ok(0),
        2 => Ok(0),
        3 => Ok(0),
        //_ => Err(Error::new(ErrorKind::Other,"some error2"))
        _ => Err(Error::from(ErrorKind::NotFound))

    }
}