use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};
pub fn read_input() -> io::Result<Vec<isize>> {
    let file = File::open("data/input05.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let numbers: Vec<_> = bb.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
    Ok(numbers)
}

use crate::intcode::{execute_instruction, parse_instruction, ER};
fn execute_until_termination(
    program: &mut Vec<isize>,
    inputs: Vec<isize>,
) -> Result<Vec<isize>, &'static str> {
    let mut ip: isize = 0;
    let mut outs: Vec<isize> = vec![];
    let mut iter = inputs.into_iter();
    loop {
        let inst = parse_instruction(&program, ip as usize)?;
        let new_ip = execute_instruction(program, ip, inst, &mut iter)?;
        match new_ip {
            ER::Terminate => break,
            ER::Continue { next, output } => {
                ip = next;
                if let Some(out) = output {
                    outs.push(out);
                }
            }
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
    let result = execute_until_termination(&mut x, vec![5]).unwrap();
    Ok(result)
}
