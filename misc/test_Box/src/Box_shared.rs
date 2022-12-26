pub mod example{
    pub fn call_me(){
        println!("box shared example");
    }

    #[derive(Debug)]
    struct Node{
        value: i32,
        next: Box<Option<Node>>,
    }
    pub fn test_nodes(){
        let a = Node{
            value: 1,
            next: Box::new(None),
        };
        let b = Node{
            value: 2,
            next: Box::new(Some(a)),
        };
        /*let b_another_a = Node{
            value: 2,
            next: Box::new(Some(a)), // moved!
        };*/


    }
}
pub mod example_Rc{
    use std::ops::{Deref, DerefMut};
    use std::{rc::Rc, borrow::BorrowMut};
    use std::rc::Weak;
    #[derive(Debug)]
    struct NodeRC{
        value: i32,
        next: Rc<Option<NodeRC>>,
    }
    pub fn test_nodes_shared(){
        let mut a = Rc::new(Some(NodeRC{ // create wrapper node
            value: 1,
            next: Rc::new(None) 
        }));
        let b = NodeRC{
            value: 2,
            next: Rc::clone(&a)
        };
        let b_another_a = NodeRC{
            value: 2,
            next: Rc::clone(&a) , // moved!
        };
        println!("strong count struct is {}",Rc::strong_count(&a)); // 3

        let mut x = Rc::new(555_i32); //1 owner
        *Rc::make_mut(&mut x) = 442; // 
        let z = Rc::clone(&x); // clone here
        println!("strong count on x is {}",Rc::strong_count(&x)); 
        {
            println!("value of weak: {}", Rc::try_unwrap(x).unwrap()  );
        }
    }
            //println!("{}", Rc::try_unwrap(x_2).unwrap() ); 

}
