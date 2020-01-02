use crate::intcode::ProgramState;
use crate::my_error::MyResult;

pub fn part1() -> MyResult<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input05.txt")?;
    let (result, k) = ps.run_with_input(&[1])?;
    assert!(k == 1);
    Ok(result)
}

pub fn part2() -> MyResult<Vec<isize>> {
    let mut ps = ProgramState::init_from_file("data/input05.txt")?;
    let (result, k) = ps.run_with_input(&[5])?;
    assert!(k == 1);
    Ok(result)
}
