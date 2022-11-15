use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};

static Gserial: AtomicUsize = AtomicUsize::new(0);
#[repr(C)]
pub struct ValueOperation {
    value: i32,
    serial: usize,
}

impl ValueOperation{
    pub fn get_operation() -> OperationResult {
        let some_int = rand::thread_rng().gen_range(0..100);
        if some_int > 50 {
            let ret_val = ValueOperation{
                value: some_int,
                serial: Gserial.fetch_add(1,Ordering::SeqCst),
            };
            return OperationResult::Success(ret_val);
        } else {
            return OperationResult::Failure;
        }
    }
    pub fn printf(&self){
        println!("value is {}, serial is {}",self.value, self.serial);
    }
    pub fn get_value(&self)-> i32{
        self.value.clone()
    }

}

pub enum OperationResult {
    Success(ValueOperation),
    Failure, 
}
