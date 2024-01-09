extern crate exif;

use std::fs::{self, rename};
use std::io::{self, Read};
use std::io::BufReader;

use exif::{Reader, Value, DateTime, In, Tag};

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
                    let file = fs::File::open(&file_path)?;
                    let exif = Reader::new().read_from_container(
                        &mut BufReader::new(&file)).unwrap();

                    // Print the file path
                    println!("File: {}", file_path.display());

                    // To parse a DateTime-like field, `DateTime::from_ascii` can be used.
                    if let Some(field) = exif.get_field(Tag::DateTimeDigitized, In::PRIMARY) {
                        match field.value {
                            Value::Ascii(ref vec) if !vec.is_empty() => {
                                if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                                    let new_filename: String = format!("{}-{}-{} - {}.{}.{}.jpg", datetime.day,
                                                                                                    datetime.month,
                                                                                                    datetime.year,
                                                                                                    datetime.hour,
                                                                                                    datetime.minute,
                                                                                                    datetime.second);
                                    println!("New filename: {}", new_filename);
                                    fs::rename(file_path, new_filename).unwrap();
                                }
                            },
                            _ => {},
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
