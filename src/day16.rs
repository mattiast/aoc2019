use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Vec<i32>> {
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

fn fft(x: &mut [i32]) {
    let n = x.len();

    let cumsum = {
        let mut cumsum = Vec::with_capacity(n + 1);
        cumsum.push(0);
        let mut s = 0;
        for v in x.iter() {
            s += v;
            cumsum.push(s);
        }
        cumsum
    };

    let f = |k: usize| cumsum[k.min(n)];

    for l in 1..n {
        let mut y = 0;
        for q in 1..n {
            if (4 * q - 3) * l >= n {
                break;
            }
            y += f((4 * q - 2) * l) - f((4 * q - 3) * l) + f((4 * q - 1) * l) - f(4 * q * l);
        }
        x[l] = y.abs() % 10;
    }
}

use clap::{value_t_or_exit, App, Arg};
pub fn part1() -> io::Result<()> {
    let matches = App::new("run computer")
        .arg(Arg::with_name("x0"))
        .arg(Arg::with_name("x1"))
        .get_matches();
    let k = value_t_or_exit!(matches, "x0", usize);
    let skip = value_t_or_exit!(matches, "x1", usize);

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
