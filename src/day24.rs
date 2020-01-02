use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn read_input() -> io::Result<u32> {
    let file = File::open("data/input24.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0u32;
    for (j, line) in reader.lines().enumerate() {
        let line = line?;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                result |= 1 << (5 * j + i);
            }
        }
    }
    Ok(result)
}

fn next(x: u32) -> u32 {
    let mut result = 0u32;

    for i in 0..25 {
        let has_bug = x & (1 << i) > 0;

        let up = (i >= 5) && (x & (1 << (i - 5)) > 0);
        let dn = (i < 20) && (x & (1 << (i + 5)) > 0);
        let lt = (i % 5 != 0) && (x & (1 << (i - 1)) > 0);
        let rt = (i % 5 != 4) && (x & (1 << (i + 1)) > 0);

        let num_neigh = (up as usize) + (dn as usize) + (lt as usize) + (rt as usize);
        let new =
            (has_bug && (num_neigh == 1)) || (!has_bug && (num_neigh <= 2) && (num_neigh >= 1));
        if new {
            result |= 1 << i;
        }
    }

    result
}

use std::collections::HashSet;
pub fn part1() -> io::Result<()> {
    let mut x = read_input()?;
    let mut seen = HashSet::new();

    println!("{}", x);
    for i in 0.. {
        if seen.contains(&x) {
            println!("Stop after {} iterations", i);
            break;
        } else {
            seen.insert(x);
        }
        x = next(x);
    }
    println!("{}", x);

    Ok(())
}
