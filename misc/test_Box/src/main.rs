use std::{rc::Rc};

mod Box_shared;
use Box_shared::example::{*};
use Box_shared::example_Rc::{*};

fn main() {
    let x= Box::new(15_i32);
    println!("value is {}, pointer {:p}",x.as_ref(),x);
    let mut xx = x.clone();
    *xx =42;
    println!("value is {}, pointer {:p}",xx.as_ref(),xx);
    let y = x; //move
    test_nodes();
    test_nodes_shared();
}

