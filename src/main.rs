extern crate rand;
use file_loader::{save, Profile};
use rand::Rng;
use std::char::{self};
use std::{io, process};

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

fn new_profile(buffer: &mut String) {
    clearscreen::clear().expect("failed to clear screen");
    buffer.clear();

    println!("enter site name");
    let _ = io::stdin().read_line(buffer);
    let site: String = String::from(buffer.trim().to_string());

    buffer.clear();

    println!("enter address");
    let _ = io::stdin().read_line(buffer);
    let address: String = String::from(buffer.trim().to_string());

    buffer.clear();

    let p: Profile = Profile {
        site,
        address,
        code: generate_code(),
    };

    let ps = serde_json::to_string(&p).unwrap();
    let pd: Profile = serde_json::from_str(&ps).unwrap(); //deserialize

    println!("{:?}", pd);

    let material: String = ps.to_string();
    let _ = save(&material);
}

fn start() -> io::Result<()> {
    clearscreen::clear().expect("failed to clear screen");
    println!("Main Menu:");
    println!("[n] = new profile");
    println!("[e] = exit");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let user_input: &str = buffer.trim();

    match user_input {
        "n" => {
            new_profile(&mut buffer);
        }
        "e" => {
            process::exit(0);
        }
        _ => {
            println!("invalid input");
        }
    }

    let _ = start();

    Ok(())
}

fn main() {
    let _ = start();
}
