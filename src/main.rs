extern crate rand;
use file_loader::{read_line, save_new_line, Profile};
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

fn new_profile(buffer: &mut String, is_custom: bool) {
    clearscreen::clear().expect("failed to clear screen");
    buffer.clear();

    println!("enter site name");
    let _ = io::stdin().read_line(buffer);
    let site: String = String::from(buffer.trim().to_string());

    buffer.clear();

    println!("\nenter address");
    let _ = io::stdin().read_line(buffer);
    let address: String = String::from(buffer.trim().to_string());

    buffer.clear();

    let code: String = {
        if !is_custom {
            generate_code()
        } else {
            println!("\nenter custom code");
            let _ = io::stdin().read_line(buffer);
            String::from(buffer.trim().to_string())
        }
    };

    let p: Profile = Profile {
        site,
        address,
        code,
    };

    let mut serialized_profile = serde_json::to_string(&p).unwrap();
    serialized_profile.push(',');
    let material: String = format!("{}\n", serialized_profile.to_string());
    let _ = save_new_line(&material);
}

fn display_profiles(buffer: &mut String) {
    clearscreen::clear().expect("failed to clear screen");
    buffer.clear();

    for i in 1..128 {
        let serialized_profile: String = read_line(i).unwrap();

        if serialized_profile != "-1".to_string() {
            println!("{}", serialized_profile);
        }
    }

    println!("press any button for main menu");
    let _ = io::stdin().read_line(buffer);
}

fn start() -> io::Result<()> {
    clearscreen::clear().expect("failed to clear screen");
    println!("Main Menu:");
    println!("[n] = new profile");
    println!("[n_c] = new custom profile");
    println!("[d] = display profiles");
    println!("[e] = exit");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let user_input: &str = buffer.trim();

    match user_input {
        "n" => {
            new_profile(&mut buffer, false);
        }
        "n_c" => {
            new_profile(&mut buffer, true);
        }
        "d" => {
            display_profiles(&mut buffer);
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
