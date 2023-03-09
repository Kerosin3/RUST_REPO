mod mod1;
mod mod2;
#[allow(unused_imports)]
use mod1::implementv1::runme1;
#[allow(unused_imports)]
use mod2::implementv1::runme2;

pub mod external {
    use super::*;
    pub fn rune(text: String) {
        println!("-------");
        runme1(text);
    }
}
