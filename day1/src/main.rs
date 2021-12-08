use std::fs;
use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let mfst_dir = env!("CARGO_MANIFEST_DIR");
    let lines = io::BufReader::new(fs::File::open(format!("{}/input", mfst_dir))?).lines();
    let mut prev: i32 = -1;
    let mut incr: i32 = 0;
    for l in lines {
        let depth = l?.parse::<i32>()?;
        if prev != -1 && depth > prev {
            incr += 1;
        }
        prev = depth;
    }
    println!("{}", incr);
    Ok(())
}
