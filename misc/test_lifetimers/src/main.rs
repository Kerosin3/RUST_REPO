//use std::intrinsics::unreachable;
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
fn main() {
    let (x,y) = (10_i32,20_i32);
    let refx = {
        let mut storage = Storage::<i32>::new();
        storage.push(&x);
        storage.push(&y);
        match storage.get(0) {
            Some(refx) => refx,
            _ => unreachable!(),
        }
    };
    println!("{refx}");
}


struct Storage<'a,T> { // жизнь как у типа
    inner: Vec<&'a T>,
}

impl <'a,T> Storage<'a,T>{
    fn new() -> Self{
        Self{
            inner:Default::default(),
        }
    }
    fn get(&self,index: usize) -> Option<&'a T>{
        self.inner.get(index).cloned()
    }
    fn push(&mut self,value: &'a T){
        self.inner.push(value);
    }
}