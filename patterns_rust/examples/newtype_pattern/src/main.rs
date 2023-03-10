// ney type pattern
fn main() {
    let mut ws = WrapString::new();
    ws.pushtext("some text".to_string());
    println!("{}", ws);
}

struct WrapString(Vec<String>);
impl WrapString {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn pushtext(&mut self, text: String) {
        self.0.push(text);
    }
}

impl std::fmt::Display for WrapString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "wrapped string foramatting method:[{}]",
            self.0.join(", ")
        )
    }
}
