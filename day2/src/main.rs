use std::fs;
use std::io::{self, BufRead};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let mfst_dir = env!("CARGO_MANIFEST_DIR");
    let lines = io::BufReader::new(fs::File::open(format!("{}/input", mfst_dir))?).lines();
    let mut h = 0;
    let mut d = 0;
    for l in lines {
        let line = l?;
        let split: Vec<&str> = line.split(" ").collect();
        let (command, x) = (split[0], split[1].parse::<i32>()?);
        match command {
            "forward" => h = h + x,
            "down" => d = d + x,
            "up" => d = d - x,
            _ => bail!("invalid command {}", command),
        }
    }
    println!("{}", h * d);
    Ok(())
}
