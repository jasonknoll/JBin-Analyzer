use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use argh::FromArgs;

use chrono::{offset::Local, DateTime};

use sha256;

#[derive(FromArgs, Debug)]
/// JBin - Binary Analysis Tool
struct Arguments {
    /// path for the file to be analyzed
    #[argh(option, short = 'p')]
    path: String,

    /// flag to set the program to display the file's hash checksum
    #[argh(switch, short = 'H')]
    hash: bool,
}

fn main() {
    /* TODO
    - [x] Import file to be analyzed
    - [x] Grab attribute data off of it
      [x]   + Size, creation date, etc.
    - [~] Hash/calculate checksum and save somewhere (idk)
    - [ ] Extract available strings
    - [ ] Find out how to disassemble an exe lol
    - [ ] Add a console GUI
    */

    let args: Arguments = argh::from_env();

    if args.path.is_empty() {
        eprintln!("Error: file path not provided! Use '-p' to enter in the path");
        std::process::exit(-1);
    }

    let file_path = Path::new(&args.path);
    get_file_metadata(file_path);

    if args.hash == true {
        hash_file(file_path);
    }
}

fn get_file_metadata(path: &Path) {
    match fs::metadata(&path) {
        Ok(metadata) => {
            let size = metadata.len();

            let create_date = metadata.created().unwrap();
            let create_date_time: DateTime<Local> = create_date.into();

            let modified_date = metadata.modified().unwrap();
            let modified_date_time: DateTime<Local> = modified_date.into();

            let file_type = path.extension().and_then(OsStr::to_str).unwrap();

            println!("Size: {} bytes", size);
            println!("Created: {}", create_date_time.format("%d/%m/%Y %X"));
            println!(
                "Last modified: {}",
                modified_date_time.format("%d/%m/%Y %X")
            );
            println!("File type: {:?}", file_type);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

// TODO - Find more things to do with the hash (compare to older versions or something)
fn hash_file(path: &Path) {
    let bytes = fs::read(path).unwrap();
    let hash = sha256::digest(&bytes);
    println!("Hash: {}", hash);
}
