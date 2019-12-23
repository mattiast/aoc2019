use crate::intcode::ProgramState;
use std::io;

pub fn part1() -> io::Result<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input09.txt")?;
    let (result, _) = ps.run_with_input(&[1]).unwrap();
    Ok(result)
}

pub fn part2() -> io::Result<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input09.txt")?;
    let (result, _) = ps.run_with_input(&[2]).unwrap();
    Ok(result)
}
