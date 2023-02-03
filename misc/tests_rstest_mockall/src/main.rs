use num::One;
use rstest::*;

fn main() {
    println!("Hello, world!");
}
#[mockall::automock]
trait MyTrait {
    fn method1(&self, x: u32) -> u32 {
        x
    }
    fn method2(&self) {
        println!("heheheh");
    }
}
// returns just a string
#[fixture]
fn return_string() -> String {
    "hello".to_string()
}
// return fixed number
#[fixture]
fn return_i32() -> i32 {
    42
}
// fixture as input to other fixture
#[fixture]
fn other_fixture_usage(return_i32: i32) -> i32 {
    return_i32 + 50_i32
}
fn call_trait_fumc(x: &dyn MyTrait) -> u32 {
    x.method1(42)
}
// ordinary function
fn add_one<T>(arg: T) -> T
where
    T: std::ops::Add<Output = T> + One,
{
    arg + One::one()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::rstest;

    #[rstest]
    #[case::no_panic(0, 1)]
    #[case(1, 2)]
    fn some_func(#[case] input: u32, #[case] summ: u32) {
        assert_eq!(summ, add_one(input));
    }

    #[rstest]
    fn test_fixture(return_string: String) {
        assert_eq!(return_string, "hello".to_string())
    }
    #[rstest]
    // add 50 and 42
    fn test_1(other_fixture_usage: i32) {
        assert_eq!(other_fixture_usage, 92_i32);
    }
    #[rstest]
    fn test_some() {
        let mut mock = MockMyTrait::new();
        mock.expect_method1()
            .with(predicate::eq(42))
            .returning(|x| x);
        assert_eq!(42_u32, mock.method1(42));
    }

    #[rstest]
    #[should_panic]
    fn let_me_panic() {
        assert!(false);
    }
    #[rstest]
    #[should_panic(expected = "something_bad_happened!")]
    fn let_me_panic_with_something() {
        panic!("something_bad_happened!")
    }

    #[rstest]
    #[case::no_panic(0)]
    #[should_panic]
    #[case::panic(1)]
    #[should_panic(expected = "expected")]
    #[case::panic_with_message(2)]
    fn attribute_per_case(#[case] val: i32) {
        match val {
            0 => assert!(true),
            1 => panic!("No catch"),
            2 => panic!("expected"),
            _ => unreachable!(),
        }
    }
}
