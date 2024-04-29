extern crate rand;
use crate::snoopy_saver::snoopy_save;
use rand::Rng;
use std::char::{self};

mod snoopy_saver;
/*
struct Profile {
    site: String,
    address: String,
    code: String,
}
*/
fn generate() -> String {
    let mut rng = rand::thread_rng();
    let mut sequence: [char; 14] = ['0'; 14];
    let mut slots_available: [bool; 14] = [true; 14];
    let mut cur_slot: i32 = rng.gen_range(0..14);

    for _ in 0..sequence.len() {
        while slots_available[cur_slot as usize] == false {
            cur_slot = rng.gen_range(0..14);
        }

        slots_available[cur_slot as usize] = false;

        match rng.gen_range(0..=3) {
            0 => sequence[cur_slot as usize] = rng.gen_range(b'A'..=b'Z') as char,
            1 => sequence[cur_slot as usize] = rng.gen_range(b'a'..=b'z') as char,
            2 => sequence[cur_slot as usize] = rng.gen_range(b'!'..=b'/') as char,
            3 => sequence[cur_slot as usize] = rng.gen_range(b'0'..=b'9') as char,
            _ => panic!("out of range"),
        }
    }

    let mut code: String = sequence[0].to_string();

    for i in 1..sequence.len() {
        code.push(sequence[i as usize]);
    }

    code
}

fn main() {
    let code: String = generate();
    let _ = snoopy_save::save(&code);
}
