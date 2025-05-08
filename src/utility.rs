use std::error::Error;
use std::fs::File;

use crate::person::Person;

pub fn read_csv_and_filter(
    column: &str,
    value: &str,
    file_path: &str,
    output_path: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut wtr = output_path
        .as_ref()
        .map(|path| csv::Writer::from_path(path))
        .transpose()?; // `wtr` is mutable here

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
            if let Some(writer) = wtr.as_mut() {
                // Borrow `wtr` as mutable
                writer.serialize(&record)?;
            }
        }
    }

    if let Some(mut writer) = wtr {
        writer.flush()?; // Flush the writer
    }

    Ok(())
}
