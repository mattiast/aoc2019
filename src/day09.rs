use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};
pub fn read_input() -> io::Result<Vec<isize>> {
    let file = File::open("data/input09.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let numbers: Vec<_> = bb.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
    Ok(numbers)
}

use crate::intcode::{execute_instruction, parse_instruction, ProgramState};
fn execute_until_termination(
    program: &mut Vec<isize>,
    inputs: Vec<isize>,
) -> Result<Vec<isize>, &'static str> {
    let mut state = ProgramState::init(program.clone());
    let mut outs: Vec<isize> = vec![];
    let mut iter = inputs.into_iter();
    let mut input = iter.next();
    while !state.terminated {
        let inst = parse_instruction(&state.mem, state.ip)?;
        if input.is_none() {
            input = iter.next();
        }
        let output = execute_instruction(&mut state, inst, &mut input)?;
        if let Some(out) = output {
            println!("Output {}", out);
            outs.push(out);
        }
    }
    assert!(iter.next() == None);
    Ok(outs)
}

pub fn part1() -> io::Result<Vec<isize>> {
    let mut x = read_input()?;
    let result = execute_until_termination(&mut x, vec![1]).unwrap();
    Ok(result)
}

pub fn part2() -> io::Result<Vec<isize>> {
    let mut x = read_input()?;
    let result = execute_until_termination(&mut x, vec![2]).unwrap();
    Ok(result)
}
