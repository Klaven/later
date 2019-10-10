use quicli::prelude::*;
use structopt::StructOpt;
#[derive(Debug,StructOpt)]
struct Cli {
    #[structopt(long = "message", short = "m")]
    message: Option<String>,
    #[structopt(long = "list", short = "l")]
    list: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    
    match args.message {
        Some(_) => println!("passed in a message"),
        _ => println!("no message"),
    }

    println!("Hello, world!");
}
