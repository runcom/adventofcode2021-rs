use std::fs;
use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let mfst_dir = env!("CARGO_MANIFEST_DIR");
    let mut lines: Vec<String> = Vec::new();
    for l in io::BufReader::new(fs::File::open(format!("{}/input", mfst_dir))?).lines() {
        lines.push(l?);
    }
    let input_ns = lines.iter().take(1);
    let mut ns: Vec<i32> = Vec::new();
    for n in input_ns {
        ns = n
            .split(",")
            .map(|v| v.to_string().parse::<i32>().unwrap())
            .collect();
    }
    let input_boards = lines.iter().skip(2);
    let mut boards: Vec<(Vec<Vec<(i32, bool)>>, bool)> = Vec::new();
    let mut board: Vec<Vec<(i32, bool)>> = Vec::new();
    for card in input_boards {
        if card != "" {
            let v: Vec<(i32, bool)> = card
                .split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| (v.to_string().trim().parse::<i32>().unwrap(), false))
                .collect();
            board.push(v);
        } else {
            if board.len() == 0 {
                continue;
            }
            boards.push((board.clone(), false));
            board.drain(..);
        }
    }
    // last board
    boards.push((board.clone(), false));

    let mut pt1_done = false;
    let mut pt2_done = false;
    let mut last_winning_score = -1;
    // this is awful...
    let mut boardsc = boards.clone();
    let mut all_won = false;
    for n in ns {
        for i in 0..boards.len() {
            fill_board(&mut boards[i].0, n);
            let (w, final_score) = check_board(&boards[i].0, n);
            if w && !pt1_done {
                pt1_done = true;
                println!("Part one: {}", final_score);
            }
            if w {
                boards[i].1 = true;
                boardsc[i].1 = true;
                last_winning_score = final_score;
            }
            for b in boardsc.iter() {
                if !b.1 {
                    all_won = false;
                    break;
                }
                all_won = true
            }
            if all_won && !pt2_done {
                pt2_done = true;
                println!("Part two: {}", last_winning_score);
            }
        }
    }
    Ok(())
}

fn check(hmv: &Vec<Vec<(i32, bool)>>, n: i32) -> (bool, i32) {
    for hm in hmv {
        let r = hm.into_iter().filter(|(_, v)| *v == true).count();
        if r != 5 {
            continue;
        }
        let mut sum = 0;
        for hm in hmv {
            let s: i32 = hm
                .into_iter()
                .filter(|(_, v)| *v == false)
                .map(|(v, _)| *v)
                .sum();
            sum = sum + s;
        }
        return (true, sum * n);
    }
    (false, -1)
}

fn check_board(hmv: &Vec<Vec<(i32, bool)>>, n: i32) -> (bool, i32) {
    let r = check(&hmv, n);
    if r.0 {
        return r;
    }
    // transpose
    let mut t: Vec<Vec<(i32, bool)>> = Vec::new();
    for _ in 0..=4 {
        t.push(Vec::new());
    }
    for j in 0..hmv.len() {
        for i in 0..hmv.len() {
            let (k, v) = hmv.into_iter().nth(i).unwrap().into_iter().nth(j).unwrap();
            t[j].push((*k, *v));
        }
    }
    check(&t, n)
}

fn fill_board(hmv: &mut Vec<Vec<(i32, bool)>>, n: i32) {
    for hm in hmv {
        for e in hm {
            if e.0 == n {
                e.1 = true;
            }
        }
    }
}
