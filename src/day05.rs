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

fn execute_instruction(program: &mut Vec<isize>, ip: isize) -> Result<Option<isize>, &'static str> {
    let inst = parse_instruction(&program, ip as usize)?;
    match inst {
        Instruction::Terminate => Ok(None),
        Instruction::Add(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            program[i3 as usize] = x1 + x2;
            Ok(Some(ip + inst.size()))
        }
        Instruction::Mul(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            program[i3 as usize] = x1 * x2;
            Ok(Some(ip + inst.size()))
        }
        Instruction::Input(i1) => {
            let hc_in = 5;
            println!("input asked, giving {}", hc_in);
            program[i1 as usize] = hc_in;
            Ok(Some(ip + inst.size()))
        }
        Instruction::Output(i1) => {
            let x1 = read_parameter(i1, program)?;
            println!("output: {}", x1);
            Ok(Some(ip + inst.size()))
        }
        Instruction::JumpIfTrue(i1, i2) => {
            let x1 = read_parameter(i1, program)?;
            if x1 != 0 {
                let x2 = read_parameter(i2, program)?;
                Ok(Some(x2))
            } else {
                Ok(Some(ip + inst.size()))
            }
        }
        Instruction::JumpIfFalse(i1, i2) => {
            let x1 = read_parameter(i1, program)?;
            if x1 == 0 {
                let x2 = read_parameter(i2, program)?;
                Ok(Some(x2))
            } else {
                Ok(Some(ip + inst.size()))
            }
        }
        Instruction::LessThan(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            let result = if x1 < x2 {1} else {0};
            program[i3 as usize] = result;
            Ok(Some(ip + inst.size()))
        }
        Instruction::Equals(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            let result = if x1 == x2 {1} else {0};
            program[i3 as usize] = result;
            Ok(Some(ip + inst.size()))
        }
    }
}

fn execute_until_termination(program: &mut Vec<isize>) -> Result<isize, &'static str> {
    let mut ip: isize = 0;
    loop {
        let new_ip = execute_instruction(program, ip)?;
        match new_ip {
            None => break,
            Some(ii) => {
                ip = ii;
            }
        }
    }
    Ok(program[0])
}

pub fn part1() -> io::Result<isize> {
    let mut x = read_input()?;
    let result = execute_until_termination(&mut x).unwrap();
    Ok(result)
}

fn execute_with_input(program: &[isize], x1: isize, x2: isize) -> Result<isize, &'static str> {
    let mut prog_copy: Vec<isize> = program.to_vec();
    prog_copy[1] = x1;
    prog_copy[2] = x2;
    execute_until_termination(&mut prog_copy)
}

use clap::{value_t_or_exit, App, Arg};
pub fn part2_manual() -> io::Result<isize> {
    let matches = App::new("run computer")
        .arg(Arg::with_name("x0"))
        .arg(Arg::with_name("x1"))
        .get_matches();
    let x0 = value_t_or_exit!(matches, "x0", isize);
    let x1 = value_t_or_exit!(matches, "x1", isize);

    let x = read_input()?;
    let result = execute_with_input(&x, x0, x1).unwrap();
    Ok(result)
}

pub fn part2() -> io::Result<(isize, isize)> {
    let x = read_input()?;
    for x1 in 0..100 {
        for x2 in 0..100 {
            let result = execute_with_input(&x, x1, x2).unwrap();
            if result == 1969_0720 {
                return Ok((x1, x2));
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "not found"))
}

#[test]
fn test_execute_instruction() {
    let mut program: Vec<isize> = vec![2, 4, 4, 5, 99, 0];
    execute_instruction(&mut program, 0).unwrap();
    assert_eq!(program[5], 9801);
}
