extern crate rand;
use file_loader::{save, Profile};
use rand::Rng;
use std::char::{self};

mod file_loader;

fn generate_code() -> String {
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
    let p: Profile = Profile {
        site: "test_site".to_string(),
        address: "test_address".to_string(),
        code: generate_code(),
    };

    let ps = serde_json::to_string(&p).unwrap();
    //let pd: Profile = serde_json::from_str(&ps).unwrap(); //deserialize

    let material: String = ps.to_string();
    let _ = save(&material);
}
