use std::env;

fn main() {
    /*let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
            } else {
        println!("enter at leas one parameter")
    }*/
    let mut s = String::from("dqweqweGzzxcvGasdasd");
    let first_word = ret_second_word(&s);
    println!("first word is {}",first_word);
}

fn ret_second_word(s: &str) -> &str{
    let mut beg:usize= 0;
    let mut end:usize= 0;
    println!("len is {}",s.len());
    let bytes = s.as_bytes();
        for (index,elem) in bytes.iter().enumerate()  {
           println!("n:{} element: {}",index,elem); 
           if *elem == b'G' && (beg!=0) {
               end = index;
           } else if *elem == b'G'{
            println!("here");
            beg = index+1;
           }
           
        }
    println!("beg is {} end is {}",beg,end);
    &s[beg..end]
    //&s[0..5]
}
