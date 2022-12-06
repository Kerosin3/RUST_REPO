use std::fmt::Debug;

fn main() {
    let z = Point{x:100};
    func1(&z);
    let q = Qwerty{t: "abcd".to_string()};
    func1(& q);
    Point::wierd();
}

struct Point{
    x: i32,
}

struct Qwerty{
    t: String,
}

pub trait SomeTrait {
    type Sometype; // associated type
    fn get_values(&self) -> Self::Sometype;
    fn wierd() where Self: Sized ;
}

impl SomeTrait for Qwerty{
    type Sometype = (String);
    fn get_values(&self) -> Self::Sometype {
        self.t.clone()
    }

    fn wierd() where Self: Sized {
        println!("wierd for qwerty");
    }
}

impl SomeTrait for Point{
    type Sometype = i32;
    fn get_values(&self) -> Self::Sometype {
        self.x
    }
    fn wierd() where Self: Sized {
        println!("wierd for point");
    }
}

pub fn func1<T>(t: &dyn SomeTrait<Sometype=(T)>) where T: Debug{
    //t.wierd();
    println!("value {:?}",t.get_values()  );
}