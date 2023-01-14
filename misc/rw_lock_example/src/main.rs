 // single writed - many readeds scene
 //
use std::sync::RwLock;
use std::sync::Arc;
use std::{time,thread};

//thread::sleep(time::Duration::from_micros(10000));
fn main() {
    let resource = Arc::new(RwLock::new(0_u16));
    let readersn = 5;
    let mut reasers_handlers = Vec::with_capacity(readersn);
    for _ in 0..readersn {
        let resource = Arc::clone(&resource);
        reasers_handlers.push(
            thread::spawn( move || {
                let mut lock_success = 0;
                let mut lock_failures = 0;
                let mut wraps = 0;
                while wraps < 100 {
                    match resource.try_read() { // 
                        Ok(guard) => {
                            lock_success+=1;
                            if *guard == 0 { //wrapped
                                wraps +=1;
                            }
                        },
                        Err(_) => {
                            lock_failures += 1;
                        }
                    }
                }
                (lock_failures,lock_success)
            } )
        )
    }
    {
        let mut loops = 0;
        while loops < 100 {
            // 100 врап адд делает врайтер
            let mut guard = resource.write().unwrap();
            *guard = guard.wrapping_add(1);
            if *guard == 0 {
                loops +=1;
            }
        }
    }
    for j in reasers_handlers{
        
        println!(" lock failures, lock success{:?}", j.join().unwrap());
    }
}
