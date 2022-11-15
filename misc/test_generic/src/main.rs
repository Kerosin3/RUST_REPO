fn main() {
    let v = vec!['a', 'b', 'c', 'd'];
    let mut s = Some::<i32>::new();
    s.push(15);

}

pub struct Some<T>{
    age: Vec<T>,
    some: Vec<T>,
}

impl<T> Some<T>{
    pub fn new()->Some<T>{
        Some { age: Vec::new(), some: Vec::new() }
    }
    pub fn push(&mut self, t:T){
        self.age.push(t);
    }
    pub fn is_empty(&self) -> bool {
        self.some.is_empty() && self.age.is_empty()
    }
    pub fn print(self,t:T) {
        for i in self.age.iter() {
            match i:
                Some(rez) => println!("rez"),
                None => ,
                _ => println!("default"),
        }
    }
}
