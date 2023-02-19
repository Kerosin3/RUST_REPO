use blake2::{digest::consts::U10, Blake2b, Digest};
use bytes::Bytes;
use chrono::prelude::*;

fn main() {
    let mut c1 = Container::new();
    println!("changing data");
    c1.assign_data("some data 1".to_string());
    println!("changing data again");
    c1.assign_data("some data 2".to_string());
    println!("debug:{:?}", c1);
}
#[derive(Debug)]
struct Container {
    data: Bytes,
    timestamp: DateTime<Utc>,
    hasher: Blake2b<U10>,
    hash: [u8; 10],
}
impl Container {
    fn new() -> Self {
        Self {
            data: Bytes::from("default_string"),
            timestamp: Utc::now(),
            hasher: Blake2b::<U10>::new(),
            hash: [0_u8; 10],
        }
    }
    // calc hash on data change
    fn assign_data(&mut self, data: String) {
        self.data = Bytes::from(data);
        self.hasher.update(self.data.clone());
        let calced_hash = self
            .hasher
            .finalize_reset()
            .as_slice()
            .try_into()
            .expect("error converting");
        let oldhash = self.hash.clone();
        self.hash = calced_hash;
        if oldhash != self.hash {
            self.notify_if_changed();
        }
    }
}

pub trait Notifier {
    fn notify_if_changed(&self);
}
impl Notifier for Container {
    fn notify_if_changed(&self) {
        println!("observer report: data has been changed!");
    }
}
