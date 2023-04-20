//https://itnext.io/rust-iterators-2f0bb958aa08
//
fn main() {
    let mut citer = CustomIter::new();
    citer.add("one".to_string());
    citer.add("two".to_string());
    citer.add("three".to_string());
    for it in citer.into_iter() {
        println!("{}", it);
    }
}

struct CustomIter {
    v: Vec<String>,
    counter: usize,
}
impl CustomIter {
    fn new() -> Self {
        CustomIter {
            v: Vec::new(),
            counter: 0,
        }
    }
    fn add(&mut self, val: String) {
        self.v.push(val);
    }
}

impl Iterator for CustomIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.v.get(self.counter) {
            Some(s) => {
                self.counter += 1;
                println!("total number {}", self.counter);
                Some(s.to_owned())
            }
            None => None,
        }
    }
}
