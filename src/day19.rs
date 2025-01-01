use clap::{arg, command};

use crate::intcode::ProgramState;
use crate::my_error::MyResult;

pub fn part1() -> MyResult<isize> {
    let ps = ProgramState::init_from_file("data/input19.txt")?;

    let mut total = 0;
    for x in 0..50 {
        for y in 0..50 {
            total += run_with_input(ps.clone(), x, y)?;
        }
    }
    Ok(total)
}

pub fn part2() -> MyResult<()> {
    let matches = command!().arg(arg!("x")).arg(arg!("y")).get_matches();
    let x = *matches.get_one::<isize>("x").unwrap();
    let y = *matches.get_one::<isize>("y").unwrap();
    let ps = ProgramState::init_from_file("data/input19.txt")?;

    println!("x lo {}", run_with_input(ps.clone(), x + 99, y)?);
    println!("x hi {}", run_with_input(ps.clone(), x + 100, y)?);
    println!("y lo {}", run_with_input(ps.clone(), x, y + 99)?);
    println!("y hi {}", run_with_input(ps.clone(), x, y + 100)?);

    Ok(())
}

fn run_with_input(mut ps: ProgramState, x: isize, y: isize) -> MyResult<isize> {
    let (output, _) = ps.run_with_input(&[x, y])?;
    assert!(output.len() == 1);
    Ok(output[0])
}
