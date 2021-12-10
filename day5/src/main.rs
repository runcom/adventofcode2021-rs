use std::fs;
use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let mfst_dir = env!("CARGO_MANIFEST_DIR");
    let mut lines: Vec<String> = Vec::new();
    for l in io::BufReader::new(fs::File::open(format!("{}/input", mfst_dir))?).lines() {
        lines.push(l?);
    }
    Ok(())
}
