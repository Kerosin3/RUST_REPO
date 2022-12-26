use std::rc::Rc;
use std::cell::RefCell;


pub trait Messenger{
    fn send(&self,msg:&str);

}
pub struct LimTracker<'a,T:'a + Messenger>{
    messenger: &'a T, // accept a messenger
    value: usize,
    max: usize,
}

impl<'a, T> LimTracker<'a,T>
where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimTracker<T>{
        LimTracker { messenger: messenger, value: 0, max }
    }
    pub fn set_value(&mut self,value:usize){
        self.value = value; //setting value
        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 1.0 {
            self.messenger.send("Ovequote")
        } else if percentage >= 0.9 {
            self.messenger.send("90% rate")
        } else if percentage >= 0.75 {
            self.messenger.send("75% rate")
        }
    }
}

fn main() {

}

#[cfg(test)]
mod tests{
    use super::*;
    use std::{cell::RefCell, borrow::Borrow};

    struct MockMsg{
        send_messages: RefCell<Vec<String>>,
    }
    impl MockMsg{
        fn new() -> Self {
            MockMsg { send_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMsg{
        fn send(&self,msg:&str) {
            self.send_messages.borrow_mut().push(String::from(msg));
        }
    }
    #[test]
    fn test_warning(){
        let mock_msg = MockMsg::new();
        let mut limit_tracker = LimTracker::new(&mock_msg, 100);
        limit_tracker.set_value(80); // new message asigns here
        println!("messages: {:?}",mock_msg.send_messages.borrow().as_slice() );
    }
}
