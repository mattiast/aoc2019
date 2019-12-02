use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};
pub fn part1() -> io::Result<i32> {
    let file = File::open("data/input01.txt")?;
    let reader = BufReader::new(file);

    let result: i32 = reader
        .lines()
        .map(|line| {
            let i: i32 = line.unwrap().parse().unwrap();
            i / 3 - 2
        })
        .sum();
    Ok(result)
}

pub fn part2() -> io::Result<i32> {
    let file = File::open("data/input01.txt")?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    for line in reader.lines() {
        let i = line?.parse::<i32>().unwrap();
        total += calc_total_fuel(i);
    }
    Ok(total)
}

fn calc_total_fuel(mut x: i32) -> i32 {
    let mut total = 0;
    while x >= 9 {
        x = x / 3 - 2;
        total += x;
    }
    total
}

#[test]
fn test_calc_total_fuel() {
    assert_eq!(calc_total_fuel(100_756), 50_346);
    assert_eq!(calc_total_fuel(1969), 966);
}
