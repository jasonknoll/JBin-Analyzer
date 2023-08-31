use std::env;

use std::fs;
use std::path::Path;

fn main() {
    /* TODO
    - Import file to be analyzed
    - Grab attribute data off of it
        + Size, creation date, etc.
    - Hash/calculate checksum and save somewhere (idk)
    - Extract available strings
    - Find out how to disassemble an exe lol
    - Add a console GUI
    */

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    // If file path is provided, great.
    // If not, wait and then ask to set it in-app.
    let mut file_path: Option<&Path> = None;

    // check for argument

    // Get basic attributes
    match fs::metadata(&file_path) {
        Ok(metadata) => {
            let size = metadata.len();

            println!("{} bytes", size);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
