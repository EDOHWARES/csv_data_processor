use std::fs::File;
use std::error::Error;

use crate::person::Person;

pub fn read_csv_and_filter(column: &str, value: &str) -> Result<(), Box<dyn Error>> {
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