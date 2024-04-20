use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("snoopster.txt")?;
    file.write_all(b"coming soon...")?;
    Ok(())
}
