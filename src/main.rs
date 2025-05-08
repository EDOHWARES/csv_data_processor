use std::env; // provides access to env variables and CL args
mod person;
mod utility;
use utility::read_csv_and_filter;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() != 3 {
        eprintln!("usage: cargo run <column> <value>");
        std::process::exit(1);
    }

    let column = &args[1];
    let value = &args[2];

    if let Err(err) = read_csv_and_filter(column, value) {
        eprintln!("Error: {}", err);
    }
}
