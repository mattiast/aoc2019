use crate::intcode::ProgramState;
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

fn get_5_stage(program: &ProgramState, phases: &[isize]) -> Result<isize, &'static str> {
    let mut x = 0;
    for phase in phases.iter() {
        let mut program = program.clone();
        let (outs, k) = program.run_with_input(&[*phase, x]).unwrap();
        assert!(k == 2);
        assert!(outs.len() == 1);
        x = outs[0];
    }
    Ok(x)
}

use permutator::Permutation;
pub fn part1() -> io::Result<isize> {
    let ps = ProgramState::init_from_file("data/input07.txt")?;

    let mut data = vec![0, 1, 2, 3, 4];
    let max_output = data
        .permutation()
        .map(|x| get_5_stage(&ps, &x).unwrap())
        .max();

    Ok(max_output.unwrap())
}

fn get_5_stage_feedback(program: &[isize], phases: &[isize]) -> Result<isize, &'static str> {
    let prog: Vec<isize> = program.to_vec();
    let mut states: Vec<_> = (0..5).map(|_| ProgramState::init(prog.clone())).collect();

    let mut inputs: Vec<Vec<isize>> = vec![
        vec![phases[0], 0],
        vec![phases[1]],
        vec![phases[2]],
        vec![phases[3]],
        vec![phases[4]],
    ];

    loop {
        for i in 0..5 {
            if states[i].terminated {
                continue;
            }
            let inst = states[i].parse_instruction().unwrap();
            if inst.needs_input() && inputs[i].is_empty() {
                continue;
            }
            let mut inp = inputs[i].first().cloned();
            let mout = states[i].execute_instruction(inst, &mut inp).unwrap();
            if !inputs[i].is_empty() && inp.is_none() {
                inputs[i].remove(0);
            }
            if let Some(out) = mout {
                let next_i = (i + 1) % 5;
                inputs[next_i].push(out);
            }
        }
        if states[4].terminated {
            break;
        }
    }

    Ok(inputs[0][0])
}

pub fn part2() -> io::Result<isize> {
    let prog = read_input()?;

    let mut data = vec![5, 6, 7, 8, 9];
    let max_output = data
        .permutation()
        .map(|x| get_5_stage_feedback(&prog, &x).unwrap())
        .max();

    Ok(max_output.unwrap())
}

#[test]
fn test_5_stage() {
    let prog = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let phases = &[1, 0, 4, 3, 2];

    assert_eq!(get_5_stage(&ProgramState::init(prog), phases), Ok(65210));
}
