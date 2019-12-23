use crate::intcode::ProgramState;
use std::io;

pub fn part1() -> io::Result<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input05.txt")?;
    let (result, k) = ps.run_with_input(&[1]).unwrap();
    assert!(k == 1);
    Ok(result)
}

pub fn part2() -> io::Result<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input05.txt")?;
    let (result, k) = ps.run_with_input(&[5]).unwrap();
    assert!(k == 1);
    Ok(result)
}
