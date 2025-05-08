use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: String,
    city: String,
}

fn read_csv_and_filter(column: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Person = result?;

        let is_match = match column {
            "name" => record.name == value,
            "age" => record.age == value,
            "city" => record.city == value,
            _ => false,
        };

        if is_match {
            println!("{:?}", record);
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

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
