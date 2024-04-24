extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn save() -> std::io::Result<()> {
    let mut file = File::create("snoopster.txt")?;
    file.write_all(b"testing")?;
    Ok(())
}

fn generate() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..10);
    println!("{}", n);
}

fn main() {
    for _n in 1..25 {
        generate();
    }
    let _ = save();
}
