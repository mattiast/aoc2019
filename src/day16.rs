use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Vec<i32>> {
    let file = File::open("data/input16.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let numbers: Vec<_> = bb.chars().map(|s| s.to_digit(10).unwrap() as i32).collect();
    Ok(numbers)
}

fn fft(x: &mut [i32]) {
    let n = x.len();

    let pattern: &[i32] = &[0, 1, 0, -1];

    for i in 0..n {
        let mut y = 0;
        for j in i..n {
            y += x[j] * pattern[(j + 1) / (i + 1) % 4];
        }
        x[i] = y.abs() % 10;
    }
}

use clap::{value_t_or_exit, App, Arg};
pub fn part1() -> io::Result<()> {
    let matches = App::new("run computer")
        .arg(Arg::with_name("x0"))
        .get_matches();
    let k = value_t_or_exit!(matches, "x0", usize);

    let mut input = read_input()?;
    let n = input.len();
    input = input.into_iter().cycle().take(k * n).collect();
    for _ in 0..100 {
        fft(&mut input);
    }
    for digit in input.iter().take(8) {
        print!("{}", digit);
    }
    println!();
    Ok(())
}
