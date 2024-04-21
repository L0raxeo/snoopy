use std::fs::File;
use std::io::prelude::*;

fn save() -> std::io::Result<()> {
    let mut file = File::create("snoopster.txt")?;
    file.write_all(b"testing")?;
    Ok(())
}

fn main() {
    let _ = save();
}
