#[macro_use]
extern crate derive_builder;
use chrono::Utc;

#[derive(Builder, Debug, PartialEq)]
struct Transaction {
    #[builder(setter(into))]
    name: String,
    #[builder(setter(into))]
    time: String,
    sender_id: usize,
    receiver_id: usize,
    #[builder(setter(into, strip_option), default)]
    special_info: Option<i32>,
    #[builder(default = "42")]
    default_info: i32,
}
fn main() {
    let transaction0 = TransactionBuilder::default()
        .name("Alex")
        .time(Utc::now().to_string())
        .sender_id(555)
        .receiver_id(777)
        .build()
        .unwrap();
    println!("{:?}", transaction0);
}
