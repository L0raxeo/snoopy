extern crate rand;
use rand::Rng;
use std::char;
use std::fs::File;
use std::io::prelude::*;

fn save() -> std::io::Result<()> {
    let mut file = File::create("snoopster.txt")?;
    file.write_all(b"testing")?;
    Ok(())
}

fn generate() {
    let mut rng = rand::thread_rng();
    let letter: char = rng.gen_range(b'A'..=b'Z') as char;
    let lowercase_letter: char = rng.gen_range(b'a'..=b'z') as char;
    let symbol: char = rng.gen_range(b'!'..=b'/') as char;
    let number: u8 = rng.gen_range(0..10);
    print!("{}{}{}{}", letter, number, symbol, lowercase_letter);
}

fn main() {
    //for _n in 1..25 {
    generate();
    //}
    let _ = save();
}
