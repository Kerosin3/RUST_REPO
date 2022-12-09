use std::fmt::Debug;
use std::any::Any;
fn main() {

    let mut vec = Vec::new();
    let a1 = TransactionSet{
        name: String::from("some string"),
        key: Box::new(String::from("key1")),
        value: Box::new(String::from("value1")),
    };
    vec.push(a1);
}
trait Details{
    type Key;
    type Value;
    fn get_detals(&self);
    fn get_key(&self) -> &Self::Key;
    fn get_value(&self) -> &Self::Value;

}

fn get<T: Any>(value: Box<dyn Any>) -> T{
    let pv = value.downcast().expect("must be of type T");
    *pv
}

#[derive(Debug)]
struct TransactionSet<T,S>{
    pub name: String,
    pub key:T,
    pub value:S

}
impl Details for TransactionSet<Box<dyn Debug>, Box<dyn Debug>>{ // любые типы реалзующие дебаг
    type Key = Box<dyn Debug>;
    type Value = Box<dyn Debug>;

    fn get_detals(&self) {
        println!("{:?} {:?} {:?}", self.name, &self.key, self.value);
    }

    fn get_key(&self) -> &Self::Key {
        &self.key
    }

    fn get_value(&self) -> &Self::Value {
        &self.value
    }

}