// https://stackoverflow.com/questions/33925232/how-to-match-over-self-in-an-enum
fn main() {
    let String_0 = "dasdasdasdas".to_string();
    let s1 = Some_enum::one(& String_0[0..5]);
    let s2 = Some_enum::two(& String_0[6..7]);
    if let Some(out) = s1.get_bytes() {
        println!("summm is {}",out);
    } else {
        println!("error!");
    }

    s1.printout();
}
 enum Some_enum<'a>{
     one(&'a str ),
     two(&'a str),
 }
// Using ref prevents the pattern matching from taking ownership of id
impl<'a> Some_enum<'a>{
    fn printout(&self){
        match *self {
            Some_enum::one(ref oo) => println!("one enum {}",*oo),
            Some_enum::two(ref oo) => println!("two enum {}",*oo),
        }
    }
    fn get_bytes(&self) -> Option<u64>{
        match *self {
            Some_enum::one(ref oo) => Some(oo.as_bytes().iter().map(|&i| i as u64).sum::<u64>()),
            Some_enum::two(ref oo) => Some(oo.as_bytes().iter().map(|&i| i as u64).sum::<u64>()),
        }
    }
}

impl<'a> Summable for Some_enum<'a>{
    fn sum(self)->u64{
        todo!()
    }
}

trait Summable {
    fn sum(self)->u64;
}