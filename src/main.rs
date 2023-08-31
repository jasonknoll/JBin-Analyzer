use std::env;

use std::fs;
use std::path::Path;

fn main() {
    /* TODO
    - [~] Import file to be analyzed
    - [ ] Grab attribute data off of it
      [ ]   + Size, creation date, etc.
    - [ ] Hash/calculate checksum and save somewhere (idk)
    - [ ] Extract available strings
    - [ ] Find out how to disassemble an exe lol
    - [ ] Add a console GUI
    */

    let args: Vec<String> = env::args().collect();

    // If file path is provided, great.
    // If not, wait and then ask to set it in-app.
    let mut file_path: Option<&Path> = None;

    // check for argument
    file_path = match args.len() {
        2 => Some(get_file_path_from_string(&args[1])),
        _ => None,
    };

    // Get basic attributes
    match file_path {
        None => println!("No file path detected. Probably should create function to get file"),
        _ => get_file_metadata(&file_path.unwrap()),
    }
}

fn get_file_path_from_string(path: &String) -> &Path {
    return Path::new(path);
}

fn get_file_path_from_user() -> &Path {
    // User input??
}

fn get_file_metadata(path: &Path) {
    match fs::metadata(&path) {
        Ok(metadata) => {
            let size = metadata.len();

            // Figure out how to format date correctly
            let create_date = metadata.created().unwrap();

            println!("Size: {} bytes", size);
            println!("Created: {:?}", create_date);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
