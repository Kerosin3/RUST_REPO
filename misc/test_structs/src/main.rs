use chrono::Utc;
//use rand::{Rng,distributions}::{Alphanumeric, DistString  };
use rand::Rng;
use rand::distributions::{Alphanumeric, DistString};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};

enum SequenseResult {
    Success(RandSequece),
    Failure,
}


static GLOBAL_DATA: Lazy<String> = Lazy::new(|| Utc::now().to_string());
static Gserial: AtomicUsize = AtomicUsize::new(0);

fn main() {
    println!("compile time is {:?}", GLOBAL_DATA.as_str());

    for _i in 0..15  {
        println!("-------------{}---------------",_i);
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
//        println!("{}", &string);
        let sqe = SeqGenerator::new(1+(_i as usize),string );
        sqe.print_data();
    }
}

#[derive(Debug)]
struct RandSequece{
    name: String,
    active: bool,
    sequence: Vec<i32> ,
    size: usize,
    serial: usize,
}

impl RandSequece{
    fn print_seq(self){
        for item in self.sequence {
           println!("element is {}",item); 
        }
    }

    fn generate(count: usize) -> SequenseResult {
        if count > 10 {
            return SequenseResult::Failure;
        }
        let mut rseq:Vec<i32> = vec![0;count]; 
        for _i in 0..count{
            rseq[_i] = rand::thread_rng().gen_range(0..100);
        }
        let rs = RandSequece{
            name: String::from("default"),
            active: true,
            sequence: rseq,
            size: count,
            serial: Gserial.fetch_add(1,Ordering::SeqCst),
        };
        return SequenseResult::Success(rs);
    }
    fn set_name(mut self,name:String) -> Self{
        self.name = name;       
        self
    }

}

#[derive(Debug)]
struct SeqGenerator {
    name: String,
    data_len: usize,
    RS: RandSequece,
}

impl SeqGenerator{

    fn new(size: usize,gen_name:String) -> Self{
        let parse_rs =RandSequece::generate(size); 
        match parse_rs {
            SequenseResult::Success(rs) => {
//            rs.set_name(gen_name.clone()); ??????????
            return SeqGenerator{
                 name: gen_name,
                 data_len: size,
                 RS: rs,
            } 
            }
            _ => panic!("error while creating sequense!"),
        }
    }
    //    let rs = RandSequece::generate(size).set_name(String::from("asdasda"));
        //rs.set_name(gen_name.clone());
        
    fn print_data(self){
        println!("name is {}",self.name);
        for i in 0..self.data_len{
            println!("sequense data: {}",self.RS.sequence[i]);
        }
        println!("serial is {}",self.RS.serial);
    }
    }


