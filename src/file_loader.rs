use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub site: String,
    pub address: String,
    pub code: String,
}

pub fn save(material: &str) -> std::io::Result<()> {
    let mut file = File::create("foo.json")?;
    file.write_all(material.as_bytes())?;
    Ok(())
}
