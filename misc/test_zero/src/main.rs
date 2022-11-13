mod process_data;
mod operation;
use operation::{ValueOperation,OperationResult};
use crate::process_data::{add_summ,SUM};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}



fn main() {
    let args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.name)
   }



    for _i in 0..30 {
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


