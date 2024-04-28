extern crate rand;
use rand::Rng;
use std::char::{self};
use std::fs::File;
use std::io::prelude::*;

fn save(sequence: &str) -> std::io::Result<()> {
    let mut file = File::create("snoopster.txt")?;
    file.write_all(sequence.as_bytes())?;
    Ok(())
}

fn generate() -> String {
    let mut rng = rand::thread_rng();
    let mut sequence: [char; 14] = ['0'; 14];
    let mut slots_available: [bool; 14] = [true; 14];
    let mut cur_slot: i32 = rng.gen_range(0..14);

    for _i in 0..14 {
        while slots_available[cur_slot as usize] == false {
            cur_slot = rng.gen_range(0..14);
        }

        slots_available[cur_slot as usize] = false;

        match rng.gen_range(0..=3) {
            0 => sequence[cur_slot as usize] = rng.gen_range(b'A'..=b'Z') as char,
            1 => sequence[cur_slot as usize] = rng.gen_range(b'a'..=b'z') as char,
            2 => sequence[cur_slot as usize] = rng.gen_range(b'!'..=b'/') as char,
            3 => sequence[cur_slot as usize] = rng.gen_range(b'0'..=b'9') as char,
            _ => println!("error"),
        }
    }

    for i in 0..sequence.len() {
        print!("{}", sequence[i]);
    }

    "blah".to_string()
}

fn main() {
    let sequence: String = generate();
    let _ = save(&sequence);
}
