use std::env; // Provides access to environment variables and CLI arguments
mod person; // Brings the person module into scope
mod utility; // Brings the utility module into scope
use utility::read_csv_and_filter; // Imports the read_csv_and_filter function from the utility module

fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Check if the number of arguments is less than 3
    if args.len() < 3 {
        // Print the expected format and exit the program with an error code
        eprintln!(
            "No be so guy!\nSee the format: cargo run <column> <value> [--file <path>] [--output <filename.csv>]"
        );
        std::process::exit(1);
    }

    // Extract the column and value arguments
    let column: &String = &args[1]; // The second argument is the column name
    let value: &String = &args[2]; // The third argument is the value to filter by

    // Set default values for file_path and output_path
    let mut file_path = "data.csv"; // Default input file is "data.csv"
    let mut output_path: Option<String> = None; // No output file by default

    // Start parsing additional arguments from index 3
    let mut i = 3;

    // Loop through the remaining arguments
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                // If "--file" is found, check if the next argument exists
                if i + 1 < args.len() {
                    file_path = &args[i + 1]; // Update file_path with the provided value
                    i += 1; // Skip the next argument since it's already processed
                }
            }
            "--output" => {
                // If "--output" is found, check if the next argument exists
                if i + 1 < args.len() {
                    output_path = Some(args[i + 1].clone()); // Update output_path with the provided value
                    i += 1; // Skip the next argument since it's already processed
                }
            }
            _ => {
                // Ignore unrecognized arguments
            }
        }
        i += 1; // Move to the next argument
    }

    // Call the read_csv_and_filter function and handle any errors
    if let Err(err) = read_csv_and_filter(column, value, file_path, output_path) {
        // Print the error message if the function fails
        eprintln!("You don jam error brr: {}", err);
    }
}
