extern crate exif;

use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let folder_path = "./test";
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "jpg" || extension == "jpeg" {
                    // Open the file
                    let file_path = entry.path();
                    let mut file = fs::File::open(&file_path)?;

                    // Read the contents of the file as bytes
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents)?;

                    // Print the file path and the contents in hexadecimal
                    println!("File: {}", file_path.display());
                }
            }
        }
    }

    Ok(())
}
