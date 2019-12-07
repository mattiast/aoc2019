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

#[derive(Clone, Copy, Debug)]
enum Parameter {
    Positional(usize),
    Immediate(isize),
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, isize),
    Mul(Parameter, Parameter, isize),
    Input(isize),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, isize),
    Equals(Parameter, Parameter, isize),
    Terminate,
}

impl Instruction {
    fn size(&self) -> isize {
        match self {
            Instruction::Add(_, _, _) => 4,
            Instruction::Mul(_, _, _) => 4,
            Instruction::Input(_) => 2,
            Instruction::Output(_) => 2,
            Instruction::JumpIfTrue(_, _) => 3,
            Instruction::JumpIfFalse(_, _) => 3,
            Instruction::LessThan(_, _, _) => 4,
            Instruction::Equals(_, _, _) => 4,
            Instruction::Terminate => 1,
        }
    }
}

fn parse_parameter(
    program: &[isize],
    addr: usize,
    mode_code: usize,
) -> Result<Parameter, &'static str> {
    let x = *program.get(addr).ok_or("out of bounds")?;
    match mode_code {
        0 => Ok(Parameter::Positional(x as usize)),
        1 => Ok(Parameter::Immediate(x)),
        _ => Err("unknown parameter mode"),
    }
}

fn parse_instruction(program: &[isize], ip: usize) -> Result<Instruction, &'static str> {
    let val0 = *program.get(ip).ok_or("out of bounds")? as usize;
    let opcode = val0 % 100;
    if opcode == 99 {
        return Ok(Instruction::Terminate);
    }
    if opcode == 1 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::Add(p1, p2, x3));
    }
    if opcode == 2 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::Mul(p1, p2, x3));
    }
    if opcode == 3 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        return Ok(Instruction::Input(x1));
    }
    if opcode == 4 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        return Ok(Instruction::Output(p1));
    }
    if opcode == 5 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        return Ok(Instruction::JumpIfTrue(p1, p2));
    }
    if opcode == 6 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        return Ok(Instruction::JumpIfFalse(p1, p2));
    }
    if opcode == 7 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::LessThan(p1, p2, x3));
    }
    if opcode == 8 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::Equals(p1, p2, x3));
    }
    Err("invalid opcode")
}

fn read_parameter(p: Parameter, program: &[isize]) -> Result<isize, &'static str> {
    match p {
        Parameter::Immediate(x) => Ok(x),
        Parameter::Positional(a) => {
            let x = program.get(a).ok_or("out of bounds")?;
            Ok(*x)
        }
    }
}

enum ER {
    Terminate,
    Continue { next: isize, output: Option<isize> },
}

impl ER {
    fn next(addr: isize) -> ER {
        ER::Continue {
            next: addr,
            output: None,
        }
    }
}

fn execute_instruction<I>(
    program: &mut Vec<isize>,
    ip: isize,
    inputs: &mut I,
) -> Result<ER, &'static str>
where
    I: Iterator<Item = isize>,
{
    let inst = parse_instruction(&program, ip as usize)?;
    match inst {
        Instruction::Terminate => Ok(ER::Terminate),
        Instruction::Add(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            program[i3 as usize] = x1 + x2;
            Ok(ER::next(ip + inst.size()))
        }
        Instruction::Mul(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            program[i3 as usize] = x1 * x2;
            Ok(ER::next(ip + inst.size()))
        }
        Instruction::Input(i1) => {
            let input = inputs.next().ok_or("ran out of inputs")?;
            program[i1 as usize] = input;
            Ok(ER::next(ip + inst.size()))
        }
        Instruction::Output(i1) => {
            let x1 = read_parameter(i1, program)?;
            Ok(ER::Continue {
                next: ip + inst.size(),
                output: Some(x1),
            })
        }
        Instruction::JumpIfTrue(i1, i2) => {
            let x1 = read_parameter(i1, program)?;
            if x1 != 0 {
                let x2 = read_parameter(i2, program)?;
                Ok(ER::next(x2))
            } else {
                Ok(ER::next(ip + inst.size()))
            }
        }
        Instruction::JumpIfFalse(i1, i2) => {
            let x1 = read_parameter(i1, program)?;
            if x1 == 0 {
                let x2 = read_parameter(i2, program)?;
                Ok(ER::next(x2))
            } else {
                Ok(ER::next(ip + inst.size()))
            }
        }
        Instruction::LessThan(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            let result = if x1 < x2 { 1 } else { 0 };
            program[i3 as usize] = result;
            Ok(ER::next(ip + inst.size()))
        }
        Instruction::Equals(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            let result = if x1 == x2 { 1 } else { 0 };
            program[i3 as usize] = result;
            Ok(ER::next(ip + inst.size()))
        }
    }
}

fn execute_until_termination(
    program: &mut Vec<isize>,
    inputs: Vec<isize>,
) -> Result<isize, &'static str> {
    let mut ip: isize = 0;
    let mut outs: Vec<isize> = vec![];
    let mut iter = inputs.into_iter();
    loop {
        let new_ip = execute_instruction(program, ip, &mut iter)?;
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

    let mut data = vec![0,1,2,3,4];
    let max_output = data.permutation().map(|x| {
        get_5_stage(&prog, &x).unwrap()
    }).max();

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
