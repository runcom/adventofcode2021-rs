use std::fs;
use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let mfst_dir = env!("CARGO_MANIFEST_DIR");
    let mut lines: Vec<String> = Vec::new();
    for l in io::BufReader::new(fs::File::open(format!("{}/input", mfst_dir))?).lines() {
        lines.push(l?);
    }
    let mut gamma_rate_v: Vec<i32> = Vec::new();
    for i in 0..=11 {
        let mut ones = 0;
        let mut zeros = 0;
        for line in &lines {
            let bit = line.chars().nth(i).unwrap().to_string().parse::<i32>()?;
            match bit {
                0 => zeros += 1,
                1 => ones += 1,
                _ => continue,
            }
        }
        if ones > zeros {
            gamma_rate_v.push(1);
        } else {
            gamma_rate_v.push(0);
        }
    }
    let gamma_rate_s: String = gamma_rate_v.iter().map(|v| v.to_string()).collect();
    let gamma_rate_i = isize::from_str_radix(&gamma_rate_s, 2).unwrap();
    let epsilon_rate_s: String = gamma_rate_v.iter().map(|v| (1 - v).to_string()).collect();
    let epsilon_rate_i = isize::from_str_radix(&epsilon_rate_s, 2).unwrap();
    println!("{}", gamma_rate_i * epsilon_rate_i);
    Ok(())
}