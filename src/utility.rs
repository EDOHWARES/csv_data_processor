use std::error::Error;
use std::fs::File;
use crate::person::Person; // Importing the Person struct from the person module

/// Reads a CSV file, filters rows based on a column and value, and optionally writes the filtered rows to an output file.
///
/// # Arguments
/// - `column`: The column name to filter by (e.g., "name", "age", "city").
/// - `value`: The value to match in the specified column.
/// - `file_path`: The path to the input CSV file.
/// - `output_path`: An optional path to the output CSV file where filtered rows will be written.
///
/// # Returns
/// - `Ok(())` if the operation is successful.
/// - `Err(Box<dyn Error>)` if an error occurs during file operations or CSV processing.
pub fn read_csv_and_filter(
    column: &str,
    value: &str,
    file_path: &str,
    output_path: Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Open the input CSV file
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file); // Create a CSV reader to parse the file

    // Create a CSV writer if an output path is provided
    let mut wtr = output_path
        .as_ref()
        .map(|path| csv::Writer::from_path(path)) // Create a writer for the output file // Result<Option<Writer<File>>, ...>>
        .transpose()?; // Handle potential errors and keep `wtr` as an Option

    // Iterate through each row in the CSV file
    for result in rdr.deserialize() {
        let record: Person = result?; // Deserialize the row into a Person struct

        // Check if the current row matches the filter criteria
        let is_match = match column {
            "name" => record.name == value,
            "age" => record.age == value,
            "city" => record.city == value,
            _ => false, // If the column is invalid, treat it as no match
        };
        

        if is_match {
            println!("{:?}", record); // Print the matching record to the console

            // If an output writer exists, write the matching record to the output file
            if let Some(writer) = wtr.as_mut() {
                writer.serialize(&record)?; // Serialize the record into the output CSV
            }
        }
    }

    // If an output writer exists, flush any remaining data to the output file
    if let Some(mut writer) = wtr {
        writer.flush()?; // Ensure all data is written to the file
    }

    Ok(()) // Return success
}
