fn main() {
    let container = ContainerBytes{
        components: vec![
            Box::new(Bytes01{
                field0: 10,
            }),
            Box::new(Bytes02{
                field0: 11,
                field1: 12,
            })
        ]
    };
    container.run();
}

pub trait Bytable { // main trait
    fn get_bytes(&self);
}

pub struct ContainerBytes{
    pub components: Vec<Box<dyn Bytable>>, // vector of objects that implements traite Bytable
}

impl ContainerBytes{ // container to iterate over
    pub fn run(&self){
        for components in self.components.iter(){
            components.get_bytes(); // dra each component
        }
    }
}
pub struct Bytes01{ // some struct that impliments trait Bytable
    field0: i32,
}

 // some struct that impliments trait Bytable
pub struct Bytes02{
    field0: i32,
    field1: i32,
}

impl Bytable for Bytes01{
    fn get_bytes(&self) {
        println!("{}",self.field0);
    }
}
impl Bytable for Bytes02{
    fn get_bytes(&self) {
        println!("{} {}",self.field0,self.field1);
    }
}