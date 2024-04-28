extern crate rand;
use rand::Rng;
use std::char;
use std::fs::File;
use std::io::prelude::*;

fn save(password: &str) -> std::io::Result<()> {
    let mut file = File::create("snoopster.txt")?;
    file.write_all(password.as_bytes())?;
    Ok(())
}

fn generate() -> String {
    let mut rng = rand::thread_rng();
    let uc_char: char = rng.gen_range(b'A'..=b'Z') as char;
    let lc_char: char = rng.gen_range(b'a'..=b'z') as char;
    let sym: char = rng.gen_range(b'!'..=b'/') as char;
    let num: u8 = rng.gen_range(0..=9);

    let result: String = format!("{}{}{}{}", uc_char, lc_char, sym, num);
    result
}

fn main() {
    let password: String = generate();
    let _ = save(&password);
    //lolol
}
