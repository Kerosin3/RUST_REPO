//! We are testing random number generator
//! just for test
use rand::Rng;
use std::io::{self,Write};
use std::io::{Error, ErrorKind};

fn main() {
    println!("Hello, world!");
    let mut v0 = fill_vec_random(10);
    print_vec_content(&v0);
    remove_less_50(&mut v0);
    print_vec_content(&v0);
    let v1 = v0; 
    let rez_f = get_value(&v1,1);
    let rez = match  rez_f{
       Ok(value) =>  println!("value some {}",value),
       Err(e)  => panic!("error {}",e),
    }; 
    //---------------------//
    let s1 = String::from("some string to test");
    let (s2,size) = str_calc_leng(s1);
    println!("size is {}",size);
    
}

/// returns passed string back and its size
///
/// # Arguments
///
/// * `s` - string
///
/// # Examples
///
/// ```
/// [TODO:example]
/// ```
fn str_calc_leng(s: String) -> (String,usize){
    let len = s.len();
    (s,len)
}

/// generate random i32 vector
///
/// # Arguments
///
/// * `size` - size of vector
///
/// # Examples
///
/// ```
/// [TODO:example]
/// ```
fn fill_vec_random(size:usize)->Vec<i32>{
    let mut vec0:Vec<i32> = vec![0;size ];
    for (_index,item) in vec0.iter_mut().enumerate()  {
        *item = rand::thread_rng().gen_range(0..100);
    }
    return vec0;

}

/// printing i32 vector content
///
/// # Arguments
///
/// * `vector` - i32 vector
///
/// # Examples
///
/// ```
/// [TODO:example]
/// ```
fn print_vec_content(vector:& Vec<i32>){
    println!("-----------------------------");
    let mut i:usize = 0;
    'check_num: loop {
        let num = vector.get(i);
        match num {
            Some(val) => {
               print!("the item is: {} \n",val) ;
               io::stdout().flush().unwrap();
            },
            None=> break 'check_num,

        }
        i+=1;
    }
}

/// removes elements lesser than 50 from i32 vec
///
/// # Arguments
///
/// * `vec` - i32 vec
///
/// # Examples
///
/// ```
/// [TODO:example]
/// ```
fn remove_less_50(vec:&mut Vec<i32>){
    println!("removing elements");
    vec.retain(|&i| i <50 );
}

/// getting a value from a vec<i32>
///
/// # Arguments
///
/// * `vec` - i32
/// * `n_elem` - num of elem to check
///
/// # Errors
///
/// Ok when element is exists, else error
///
/// # Examples
///
/// ```
/// [TODO:example]
/// ```
//fn get_value(vec: & Vec<i32>,n_elem:usize) -> Result<i32, &'static str >{
fn get_value(vec: & Vec<i32>,n_elem:usize) -> Result<i32, Error >{
    let out = vec.get(n_elem);
    if ! out.is_none() {
        Ok(*out.unwrap())
    } else {
        Err(Error::new(ErrorKind::Other, "oh no!"))
    }
}
