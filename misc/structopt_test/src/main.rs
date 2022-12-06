use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic",
author= "Alex",
about = "test example")]

struct CLI {
    #[structopt(short, long,global=true, help="debug info")] // -d or --debuf
    debug: bool,
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    //#[structopt(short, long, parse(from_occurrences))]
    //verbose: u8,
    input: String,
    //#[structopt(long, short, default_value = "2", help = "number of iterations")]
    //inter: u8,
    #[structopt(subcommand)]
    cmd: SubCommand
}
#[derive(Debug,StructOpt)]
enum SubCommand {
    #[structopt(name = "mod", about = "Use mod to modify strings")]
    Modify(ModifyOptions),
    #[structopt(name = "insp", about = "Use insp to inspect strings")]
    Inspect(InspectOptions)
}
#[derive(Debug,StructOpt)]
struct ModifyOptions{
    #[structopt(long,short,help="turn into lowercase")]
    lower: bool,
    #[structopt(long,short,help="turn into uppercase")]
    upper: bool,
    #[structopt(short="pref", long, help = "Adds a prefix to the string")]
    prefix: Option<String>,
    #[structopt(short="suf", long, help = "Adds a suffix to the string")]
    suffix: Option<String>,
}
#[derive(Debug,StructOpt)]
struct InspectOptions {
    #[structopt(short, long, help = "Count all characters in the string")]
    length: bool,
    #[structopt(short, long, help = "Count only numbers in the given string")]
    numbers: bool,
    #[structopt(short, long, help = "Count all spaces in the string")]
    spaces: bool
}

fn modify(input: &String, debug: bool, args: &ModifyOptions) {
    println!("Inspect called for {}", input);
    if debug {
        println!("{:#?}", args);
    }
}

fn inspect(input: &String, debug: bool, args: &InspectOptions) {
    println!("Inspect called for {}", input);
    if debug {
        println!("{:#?}", args);
    }
}

fn main(){
    let args = CLI::from_args();
    match args.cmd {
        SubCommand::Inspect(opt) => {
            inspect(&args.input, args.debug, &opt);
        }
        SubCommand::Modify(opt) => {
            modify(&args.input, args.debug, &opt);
        }

    }
}
