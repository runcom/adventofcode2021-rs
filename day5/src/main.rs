use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let mfst_dir = env!("CARGO_MANIFEST_DIR");
    let mut lines: Vec<String> = Vec::new();
    for l in io::BufReader::new(fs::File::open(format!("{}/input", mfst_dir))?).lines() {
        lines.push(l?);
    }
    let mut entries: Vec<((i32, i32), (i32, i32))> = Vec::new();
    for l in &lines {
        let ns: Vec<String> = l.split(" -> ").map(|v| v.to_string()).collect();
        let x1y1_v: Vec<i32> = ns[0]
            .split(",")
            .map(|v| v.to_string().trim().parse::<i32>().unwrap())
            .collect();
        let x2y2_v: Vec<i32> = ns[1]
            .split(",")
            .map(|v| v.to_string().trim().parse::<i32>().unwrap())
            .collect();
        let entry: ((i32, i32), (i32, i32)) = ((x1y1_v[0], x1y1_v[1]), (x2y2_v[0], x2y2_v[1]));
        entries.push(entry);
    }
    let mut coordinates: HashMap<(i32, i32), i32> = HashMap::new();
    let mut overlapping_coordinates = 0;
    let equal_x_entries: Vec<&((i32, i32), (i32, i32))> =
        entries.iter().filter(|e| e.0 .0 == e.1 .0).collect();
    for i in equal_x_entries {
        let from_x = i.1 .0;
        let from_y = i.0 .1;
        let to_y = i.1 .1;
        let mut difference = from_y - to_y;
        if difference < 0 {
            difference *= -1;
        }
        let mut smaller_y = from_y;
        if from_y > to_y {
            smaller_y = to_y;
        }
        for i in 0..=difference {
            let c = (from_x, smaller_y + i);
            if coordinates.contains_key(&c) {
                let count = coordinates.entry(c).or_default();
                if *count == 2 {
                    continue;
                }
                *count += 1;
                overlapping_coordinates += 1;
            } else {
                coordinates.insert(c, 1);
            }
        }
    }
    let equal_y_entries: Vec<&((i32, i32), (i32, i32))> =
        entries.iter().filter(|e| e.0 .1 == e.1 .1).collect();
    for i in equal_y_entries {
        let from_y = i.0 .1;
        let from_x = i.0 .0;
        let to_x = i.1 .0;
        let mut difference = from_x - to_x;
        if difference < 0 {
            difference *= -1;
        }
        let mut smaller_x = from_x;
        if from_x > to_x {
            smaller_x = to_x;
        }
        for i in 0..=difference {
            let c = (smaller_x + i, from_y);
            if coordinates.contains_key(&c) {
                let count = coordinates.entry(c).or_default();
                if *count == 2 {
                    continue;
                }
                *count += 1;
                overlapping_coordinates += 1;
            } else {
                coordinates.insert(c, 1);
            }
        }
    }
    println!("Part one: {}", overlapping_coordinates);
    let diagonals: Vec<&((i32, i32), (i32, i32))> = entries
        .iter()
        .filter(|e| !(e.0 .1 == e.1 .1) && !(e.0 .0 == e.1 .0))
        .collect();
    for e in diagonals {
        if e.0 .0 < e.1 .0 && e.0 .1 < e.1 .1 {
            let mut c = (e.1 .0, e.1 .1);
            if coordinates.contains_key(&c) {
                let count = coordinates.entry(c).or_default();
                if *count != 2 {
                    *count += 1;
                    overlapping_coordinates += 1;
                }
            } else {
                coordinates.insert(c, 1);
            }
            while c != (e.0 .0, e.0 .1) {
                let x = c.0 - 1;
                let y = c.1 - 1;
                c = (x, y);
                if coordinates.contains_key(&c) {
                    let count = coordinates.entry(c).or_default();
                    if *count == 2 {
                        continue;
                    }
                    *count += 1;
                    overlapping_coordinates += 1;
                } else {
                    coordinates.insert(c, 1);
                }
            }
        }
        if e.0 .0 > e.1 .0 && e.0 .1 > e.1 .1 {
            let mut c = (e.1 .0, e.1 .1);
            if coordinates.contains_key(&c) {
                let count = coordinates.entry(c).or_default();
                if *count != 2 {
                    *count += 1;
                    overlapping_coordinates += 1;
                }
            } else {
                coordinates.insert(c, 1);
            }
            while c != (e.0 .0, e.0 .1) {
                let x = c.0 + 1;
                let y = c.1 + 1;
                c = (x, y);
                if coordinates.contains_key(&c) {
                    let count = coordinates.entry(c).or_default();
                    if *count == 2 {
                        continue;
                    }
                    *count += 1;
                    overlapping_coordinates += 1;
                } else {
                    coordinates.insert(c, 1);
                }
            }
        }
        if e.0.0 > e.1.0 && e.0.1 < e.1.1 {
            let mut c = (e.1 .0, e.1 .1);
            if coordinates.contains_key(&c) {
                let count = coordinates.entry(c).or_default();
                if *count != 2 {
                    *count += 1;
                    overlapping_coordinates += 1;
                }
            } else {
                coordinates.insert(c, 1);
            }

            while c != (e.0 .0, e.0 .1) {
                let x = c.0 + 1;
                let y = c.1 - 1;
                c = (x, y);
                if coordinates.contains_key(&c) {
                    let count = coordinates.entry(c).or_default();
                    if *count == 2 {
                        continue;
                    }
                    *count += 1;
                    overlapping_coordinates += 1;
                } else {
                    coordinates.insert(c, 1);
                }
            }
        }
        if e.0.0 < e.1.0 && e.0.1 > e.1.1 {
            let mut c = (e.1 .0, e.1 .1);
            if coordinates.contains_key(&c) {
                let count = coordinates.entry(c).or_default();
                if *count != 2 {
                    *count += 1;
                    overlapping_coordinates += 1;
                }
            } else {
                coordinates.insert(c, 1);
            }

            while c != (e.0 .0, e.0 .1) {
                let x = c.0 - 1;
                let y = c.1 + 1;
                c = (x, y);
                if coordinates.contains_key(&c) {
                    let count = coordinates.entry(c).or_default();
                    if *count == 2 {
                        continue;
                    }
                    *count += 1;
                    overlapping_coordinates += 1;
                } else {
                    coordinates.insert(c, 1);
                }
            }
        }
    }
    println!("Part two: {}", overlapping_coordinates);
    Ok(())
}
