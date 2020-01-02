use crate::intcode::ProgramState;
use crate::my_error::MyResult;

pub fn part1() -> MyResult<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input09.txt")?;
    let (result, _) = ps.run_with_input(&[1])?;
    Ok(result)
}

pub fn part2() -> MyResult<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input09.txt")?;
    let (result, _) = ps.run_with_input(&[2])?;
    Ok(result)
}
