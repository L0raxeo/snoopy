use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};

pub static NULL_LINE: &str = "-1";

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub site: String,
    pub address: String,
    pub code: String,
}

pub fn read(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

pub fn save_full_file(material: &str) -> io::Result<()> {
    let mut save_file = File::create("foo.json")?;
    save_file.write_all(material.as_bytes())?;
    Ok(())
}

pub fn save_new_line(material: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("foo.json")?;

    file.write_all(material.as_bytes())?;

    Ok(())
}

pub fn read_line(target_line_number: u8) -> io::Result<String> {
    let file = File::open("foo.json")?;
    let reader = io::BufReader::new(file);

    let mut line_number = 1;
    for line_result in reader.lines() {
        let line = line_result?;
        if line_number == target_line_number {
            return Ok(line); // Return the line as a Result<String>
        }

        line_number += 1;
    }

    Ok(NULL_LINE.to_string())
}
