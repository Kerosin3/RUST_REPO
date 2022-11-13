mod process_data;
mod operation;
use operation::{ValueOperation,OperationResult};
use crate::process_data::{add_summ,SUM};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input isize
   #[arg(short, long)]
   ncount: i32,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}



fn main() {
    let mut args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.ncount)
   }
   if args.ncount == 0 {
       args.ncount = 1;
   } else if args.ncount >=30 {
       args.ncount = 30;
       println!("setting count to 30");
   }

    for _i in 0..args.ncount{
    print!("iteration is [{}] ",_i);
    let operation = operation::ValueOperation::get_operation();
    match operation {
        OperationResult::Success(operation) => {
            operation.printf();
            add_summ(operation.get_value());
        }

        OperationResult::Failure=> println!("failure while getting a value"),
        }
    }
    println!("summ of all values is {:?}",SUM);
}


