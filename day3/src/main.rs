use std::fs;
use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let mfst_dir = env!("CARGO_MANIFEST_DIR");
    let mut lines: Vec<String> = Vec::new();
    for l in io::BufReader::new(fs::File::open(format!("{}/input", mfst_dir))?).lines() {
        lines.push(l?);
    }
    let gamma_rate_v = find_rate(lines.clone())?;
    let gamma_rate_s: String = gamma_rate_v.iter().map(|v| v.to_string()).collect();
    let gamma_rate_i = isize::from_str_radix(&gamma_rate_s, 2).unwrap();
    let epsilon_rate_s: String = gamma_rate_v.iter().map(|v| (1 - v).to_string()).collect();
    let epsilon_rate_i = isize::from_str_radix(&epsilon_rate_s, 2).unwrap();
    println!("Part one: {}", gamma_rate_i * epsilon_rate_i);
    let mut oxygen_generator_rating_v: Vec<String> = lines.clone();
    let mut rate_v = gamma_rate_v.clone();
    for i in 0..=11 {
        let tmp = filter_vec(oxygen_generator_rating_v.clone(), rate_v[i], i)?;
        if tmp.len() == 0 {
            break
        }
        oxygen_generator_rating_v = tmp.clone();
        rate_v = find_rate(oxygen_generator_rating_v.clone())?;
    }
    let mut co2_scrubber_rating_v: Vec<String> = lines.clone();
    let epsilon_rate_v: Vec<i32> = gamma_rate_v.iter().map(|v| (1 - v)).collect();
    let mut rate_v = epsilon_rate_v.clone();
    for i in 0..=11 {
        let tmp = filter_vec(co2_scrubber_rating_v.clone(), rate_v[i], i)?;
        if tmp.len() == 0 {
            break
        }
        co2_scrubber_rating_v = tmp;
        rate_v = find_rate(co2_scrubber_rating_v.clone())?;
        rate_v = rate_v.iter().map(|v| 1 - v).collect();
    }
    let oxygen_generator_rating_s: String = oxygen_generator_rating_v.iter().map(|v| v.to_string()).collect();
    let oxygen_generator_rating_i = isize::from_str_radix(&oxygen_generator_rating_s, 2).unwrap();
    let co2_scrubber_rating_s: String = co2_scrubber_rating_v.iter().map(|v| v.to_string()).collect();
    let co2_scrubber_rating_i = isize::from_str_radix(&co2_scrubber_rating_s, 2).unwrap();
    println!("Part two: {}", oxygen_generator_rating_i * co2_scrubber_rating_i);

    Ok(())
}

fn find_rate(v: Vec<String>) -> Result<Vec<i32>> {
    let mut rate_v: Vec<i32> = Vec::new();
    for i in 0..=11 {
        let mut ones = 0;
        let mut zeros = 0;
        for line in &v {
            let bit = line.chars().nth(i).unwrap().to_string().parse::<i32>()?;
            match bit {
                0 => zeros += 1,
                1 => ones += 1,
                _ => continue,
            }
        }
        if ones == zeros {
            rate_v.push(1);
        } else {
            if ones > zeros {
                rate_v.push(1);
            } else {
                rate_v.push(0);
            }
        }
    }
    Ok(rate_v)
}

fn filter_vec(v: Vec<String>, bit: i32, pos: usize) -> Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();
    for line in &v {
        if line.chars().nth(pos).unwrap().to_string().parse::<i32>()? == bit {
            res.push(line.clone())
        }
    }
    Ok(res.clone())
}