use std::ops::Add; // import Add trait
use std::fmt::{Display,Formatter};
use std::process::Output;
fn main() {
  //  let mut x = vec![10,20,32,5,3,23];
  //  println!("value is {}",largets(&x));
    let po = Point{x:5,y:10,serial:10}; 
    let p1 = Point{x:15,y:20,serial:20}; 
//    let x = po.add(p1);
    let z = po + p1;
    println!("value is {}",z);
    z.get_def_info();
}
#[derive(Copy,Clone,PartialEq,Debug)]
struct Point<T,U>{
    x: T,
    y: U,
    serial: T,
}
/*
impl<T,U> Point<T,U>{
    fn ss<T: Copy>(&self)->T{
        self.x + self.y
    }
}
*/
impl <T,U: Copy> Point<T,U> 
where T: Display+Copy,
      U: Display+Copy,
{
    fn get_self(&self) -> Point<T,U>{
        Point { x: self.x,
                serial: self.serial,
                y: self.y}
    }
    fn printout(&self) {
        println!("value is {}",self.x);
    }
}
impl<T,U> Add for Point<T,U>
where T: Add<Output = T>,
      U: Add<Output = U>,
{
    type Output = Self;
    fn add(self,other: Self) -> Self::Output{
        Self{
            x: self.x + other.x,
            serial: self.serial,
            y: self.y + other.y,
        }
    }
}

impl<T,U> Display for Point<T,U> 
where T: Display,
      U: Display, 
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y ,self.serial)
    }
}

impl<T,U> Summ<T,U> for Point<T,U> 
where T: Add + Copy,
      U: Add, 
{
    fn get_def_info(&self) {
        println!("overrides!");
    }
}


pub trait Summ<T,U> {
    //type X = T;
    //fn summ_all(&self) -> T::Output;
    fn get_def_info(&self){
        println!("somestring");
    }
}

/*
fn largets<T: PartialOrd + Copy >(list: &[T]) -> T {
    let mut largets  = list[0];
    for &item in list.iter(){
        if item > largets{
            largets = item;
        }
    }
    largets
}*/
