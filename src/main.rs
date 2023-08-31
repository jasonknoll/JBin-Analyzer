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

    // Get basic attributes
    let file_path = Path::new("Cargo.toml");

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
