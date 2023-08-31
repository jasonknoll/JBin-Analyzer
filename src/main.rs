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

    // If file path is provided, great.
    // If not, wait and then ask to set it in-app.
    let mut file_path: Option<&Path> = None;

    // check for argument
    file_path = match args.len() {
        2 => Some(get_file_path(&args[1])),
        _ => None,
    };
    
    // Get basic attributes
    match file_path {
        None => println!("No file path detected"),
        _ => get_file_metadata(&file_path.unwrap())
    }
}

fn get_file_path(path: &String) -> &Path{
    return Path::new(path);
}

fn get_file_metadata(path: &Path){
    match fs::metadata(&path) {
        Ok(metadata) => {
            let size = metadata.len();

            println!("{} bytes", size);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
