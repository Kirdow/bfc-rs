use std::fs::File;
use std::io::{Error, Read, Write};

pub fn read_file(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut result: String = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

pub fn write_file(file_path: &str, content: &String) -> Result<(), Error> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}