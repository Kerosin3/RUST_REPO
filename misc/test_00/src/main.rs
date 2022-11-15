#[derive(Clone, Debug)]
struct Some1 {
    some: i32,
    index: usize,
}

fn main(){
    let x = get_11(10);
    println!("b = {:?}",x.get(1));
}

fn get_11(size: usize) ->Vec<Some1>{
    let mut out: Vec<Some1> = Vec::new();
    for i in  0..size {
       let s1 = Some1{some:666_i32,index:i};
       out.push(s1); 
    }
    out
}
