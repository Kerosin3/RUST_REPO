use std::fmt::Display;

trait Printer<S: Display> {
    fn display(&self, to_print: S) {
        println!("{}", to_print);
    }
}
#[derive(Default)]
struct ActualPrinter1 {}

#[derive(Default)]
struct ActualPrinter2 {}

//overrides default impl
impl<T: Display> Printer<T> for ActualPrinter1 {
    fn display(&self, to_print: T) {
        println!("printing  string from Printer 1 [{}]", to_print);
    }
}

// the same in other form
impl<T> Printer<T> for ActualPrinter2
where
    T: Display,
{
    fn display(&self, to_print: T) {
        println!("printing an integer from Printer 2 [{}]", to_print);
    }
}

//impl<S: Display, T> Printer<S> for T {} // will conflicts

#[test]
fn test_trait() {
    let st1 = "some string".to_string();
    ActualPrinter1::default().display(st1);
    ActualPrinter2::default().display(5);
}
