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

use clap::{value_t_or_exit, App, Arg};
pub fn part2() -> MyResult<()> {
    let matches = App::new("run computer")
        .arg(Arg::with_name("x"))
        .arg(Arg::with_name("y"))
        .get_matches();
    let x = value_t_or_exit!(matches, "x", isize);
    let y = value_t_or_exit!(matches, "y", isize);
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
