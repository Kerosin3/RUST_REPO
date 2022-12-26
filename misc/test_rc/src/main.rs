#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]

use std::rc::{Rc,Weak};
use std::{cell::RefCell, borrow::Borrow};
use std::cell::Cell;


struct MyStruct{
    v: i32,
}

impl MyStruct {
    fn new(value:i32) -> Self {
        MyStruct { v: value }
    }
}

impl Trait for MyStruct{
    fn get(&self) -> i32 {
        self.v
    }
    fn change(&mut self) {
        self.v += 1;
    }
}

trait Trait {
    fn change(&mut self);
    fn get(&self) -> i32;
}

fn main() {
    let x = Rc::new(RefCell::new( MyStruct::new(0) ))    ;
    println!("value is {}",x.as_ref().borrow().get()); // unwrap to &RefCell
    {
        let y = Rc::clone(&x); //clone
        y.as_ref().borrow_mut().change(); // change
    }
    println!("value is {}",x.as_ref().borrow().get()); // watch value changes
    x.as_ref().borrow_mut().change();// change one more time
    println!("changed value is {}",x.as_ref().borrow().get()); // watch value changes
    //-----------------------//
    let mut a = Rc::new(MyStruct::new(10));// strong Rc
    println!("a value is {}",a.get());
    let b = Rc::downgrade(&a); // Weak, not responsible for deallocation
    assert_eq!(b.upgrade().unwrap().v,10); // upgrade and get value
    //let x = Rc::get_mut(&mut a).unwrap();
    //x.v = 10;


    let x = Cell::new(42);// like immutable!
    println!("value inside cell {}",x.get());
    x.set(15); // we can set it now
    println!("value inside cell {}",x.get());
}


