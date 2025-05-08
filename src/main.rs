use std::env;
mod person;
mod utility;
use utility::read_csv_and_filter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "No be so guy!\nSee the format: cargo run <column> <value> [--file <path>] [--output <filename.csv>]"
        );
        std::process::exit(1);
    }

    let column: &String = &args[1];
    let value: &String = &args[2];

    let mut file_path = "data.csv";
    let mut output_path: Option<String> = None;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                println!("filing..");
                if i + 1 < args.len() {
                    file_path = &args[i + 1];
                    i += 1;
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if let Err(err) = read_csv_and_filter(column, value, file_path, output_path) {
        eprintln!("You don jam error brr: {}", err);
    }
}
