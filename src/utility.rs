use std::error::Error;
use std::fs::File;

use crate::person::Person;

pub fn read_csv_and_filter(
    column: &str,
    value: &str,
    output_path: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut matched_records: Vec<Person> = Vec::new();

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
            matched_records.push(record);
        }
    }

    // Save to output file if provided
    if let Some(path) = output_path {
        let mut wtr = csv::Writer::from_path(path)?;
        for record in matched_records {
            wtr.serialize(record)?;
        }
        wtr.flush()?;
        println!("Filtered result saved to file.");
    }

    Ok(())
}
