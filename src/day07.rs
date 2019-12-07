use crate::intcode::{execute_instruction, parse_instruction, ER, ProgramState};
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Vec<isize>> {
    let file = File::open("data/input07.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let numbers: Vec<_> = bb.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
    Ok(numbers)
}

fn execute_until_termination(
    program: &mut Vec<isize>,
    inputs: Vec<isize>,
) -> Result<isize, &'static str> {
    let mut state = ProgramState::init(program.clone());
    let mut outs: Vec<isize> = vec![];
    let mut iter = inputs.into_iter();
    loop {
        let inst = parse_instruction(&state.mem, state.ip)?;
        let new_ip = execute_instruction(&mut state, inst, &mut iter)?;
        match new_ip {
            ER::Terminate => break,
            ER::Continue { output } => {
                if let Some(out) = output {
                    outs.push(out);
                }
            }
        }
    }
    assert!(iter.next() == None);
    assert!(outs.len() == 1);
    Ok(outs[0])
}

fn get_output(program: &[isize], phase: isize, input: isize) -> Result<isize, &'static str> {
    let mut prog_copy = Vec::from(program);
    execute_until_termination(&mut prog_copy, vec![phase, input])
}

fn get_5_stage(program: &[isize], phases: &[isize]) -> Result<isize, &'static str> {
    let x1 = get_output(&program, phases[0], 0)?;
    let x2 = get_output(&program, phases[1], x1)?;
    let x3 = get_output(&program, phases[2], x2)?;
    let x4 = get_output(&program, phases[3], x3)?;
    let x5 = get_output(&program, phases[4], x4)?;
    Ok(x5)
}

use permutator::Permutation;
pub fn part1() -> io::Result<()> {
    let prog = read_input()?;

    let mut data = vec![0, 1, 2, 3, 4];
    let max_output = data
        .permutation()
        .map(|x| get_5_stage(&prog, &x).unwrap())
        .max();

    println!("{:?}", max_output);

    Ok(())
}

#[test]
fn test_5_stage() {
    let prog = &[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let phases = &[1, 0, 4, 3, 2];

    assert_eq!(get_5_stage(prog, phases), Ok(65210));
}
