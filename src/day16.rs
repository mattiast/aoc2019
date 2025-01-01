use clap::{arg, command};

use crate::my_error::MyResult;
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

pub fn read_input() -> MyResult<Vec<i32>> {
    let file = File::open("data/input16.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let mut numbers: Vec<_> = bb.chars().map(|s| s.to_digit(10).unwrap() as i32).collect();
    numbers.insert(0, 0);
    numbers.remove(650);
    Ok(numbers)
}

fn fft(xs: &mut [i32]) {
    let n = xs.len();

    let cumsum = {
        let mut cumsum = Vec::with_capacity(n + 1);
        cumsum.push(0);
        let mut s = 0;
        for v in xs.iter() {
            s += v;
            cumsum.push(s);
        }
        cumsum
    };

    let f = |k: usize| cumsum[k.min(n)];

    for (l, x) in xs.iter_mut().enumerate().skip(1) {
        let mut y = 0;
        for q in 1..n {
            if (4 * q - 3) * l >= n {
                break;
            }
            y += f((4 * q - 2) * l) - f((4 * q - 3) * l) + f((4 * q - 1) * l) - f(4 * q * l);
        }
        *x = y.abs() % 10;
    }
}

pub fn part1() -> MyResult<()> {
    let matches = command!().arg(arg!("x0")).arg(arg!("x1")).get_matches();
    let k = *matches.get_one::<usize>("x0").unwrap();
    let skip = *matches.get_one::<usize>("x1").unwrap();

    let mut input = read_input()?;
    let n = input.len();
    input = input.into_iter().cycle().take(k * n).collect();
    for _ in 0..100 {
        fft(&mut input);
    }
    for digit in input.iter().skip(skip + 1).take(8) {
        print!("{}", digit);
    }
    println!();
    Ok(())
}
