use std::sync::atomic::{AtomicI32, Ordering};

pub static SUM: AtomicI32 = AtomicI32::new(0);

pub fn add_summ(val:i32){
    SUM.fetch_add(val,Ordering::SeqCst);
}
