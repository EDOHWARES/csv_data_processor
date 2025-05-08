use std::env; // provides access to env variables and CL args
mod person;
mod utility;
use utility::read_csv_and_filter;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 3 {
        eprintln!("No be so guy!\nSee the format: cargo run <column> <value> [--output output.csv]");
        std::process::exit(1);
    }

    let mut output_path: Option<String> = None;

    if args.len() == 5 && args[3] == "--output" {
        output_path = Some(args[4].clone());
    }

    let column = &args[1];
    let value = &args[2];

    if let Err(err) = read_csv_and_filter(column, value, output_path) {
        eprintln!("You don jam error: {}", err);
    }
}
