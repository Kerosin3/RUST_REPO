use std::io::Read;

fn main() {
    let x: u64 = 1230312312353454;
    println!("==={}", ret_v(x));
}
// get digit as number
fn ss(x: i32) -> Vec<u8> {
    //let s_out = x.to_string();
    x.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as u8))
        .collect::<Vec<u8>>()
}
fn ret_v(x: u64) -> u64 {
    let mut out: Vec<u32> = Vec::new();
    for i in x
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
    {
        out.push(i);
    }
    out.sort();
    out.reverse();
    out.iter()
        .fold(0_u64, |acc, elem| acc * 10 + (*elem as u64))
}
/*
fn ret_v(x: u64) -> u64{
    let mut out: Vec<u32> = Vec::new();
    for i in x
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as u32))
    {
        out.push(i);
    }
    out.sort();
    out.reverse();
    println!("{:?}", out);
    println!(
        "{:?}",
        out.iter()
            .fold(0_u64, |acc, elem| acc * 10 + (*elem as u64))
    );
    out.iter()
            .fold(0_u64, |acc, elem| acc * 10 + (*elem as u64))

}
*/

fn somenum(x: u64) {
    let byte_rep = x.to_be_bytes();
    let mut u64_out = vec![0_u8; 8];
    let mut max: u8 = byte_rep[0];
    for &i in byte_rep.iter() {
        println!("value is {:#04x}", &i);
        println!("value is {}", &i);
    }
}
