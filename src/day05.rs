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

#[derive(Clone, Copy)]
enum ParameterMode {
    Positional,
    Immediate,
}

enum Instruction {
    Add((usize, ParameterMode), (usize, ParameterMode), usize),
    Mul((usize, ParameterMode), (usize, ParameterMode), usize),
    Input(usize),
    Output((usize, ParameterMode)),
    Terminate,
}

impl Instruction {
    fn size(&self) -> usize {
        match self {
            Instruction::Add(_,_,_) => 4,
            Instruction::Mul(_,_,_) => 4,
            Instruction::Input(_) => 2,
            Instruction::Output(_) => 2,
            Instruction::Terminate => 1,
        }
    }
}

fn parse_parameter_mode(x: usize) -> Result<ParameterMode, &'static str> {
    match x {
        0 => Ok(ParameterMode::Positional),
        1 => Ok(ParameterMode::Immediate),
        _ => Err("unknown parameter mode"),
    }
}

fn parse_instruction(program: &[usize], ip: usize) -> Result<Instruction, &'static str> {
    let val0 = *program.get(ip).ok_or("out of bounds")?;
    let opcode = val0 % 100;
    if opcode == 99 {
        return Ok(Instruction::Terminate);
    }
    if opcode == 1 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        let pm1 = parse_parameter_mode((val0 / 100) % 10)?;
        let x2 = *program.get(ip + 2).ok_or("out of bounds")?;
        let pm2 = parse_parameter_mode((val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::Add((x1, pm1), (x2, pm2), x3));
    }
    if opcode == 2 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        let pm1 = parse_parameter_mode((val0 / 100) % 10)?;
        let x2 = *program.get(ip + 2).ok_or("out of bounds")?;
        let pm2 = parse_parameter_mode((val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::Mul((x1, pm1), (x2, pm2), x3));
    }
    if opcode == 3 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        return Ok(Instruction::Input(x1));
    }
    if opcode == 4 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        let pm1 = parse_parameter_mode((val0 / 100) % 10)?;
        return Ok(Instruction::Output((x1, pm1)));
    }
    Err("invalid opcode")
}

fn read_parameter((a, pm): (usize, ParameterMode), program: &[usize]) -> Result<usize, &'static str> {
    match pm {
        ParameterMode::Immediate => Ok(a),
        ParameterMode::Positional => {
            let x1 = program.get(a).ok_or("out of bounds")?;
            Ok(*x1)
        }
    }
}

fn execute_instruction(program: &mut Vec<usize>, ip: usize) -> Result<Option<usize>, &'static str> {
    let inst = parse_instruction(&program, ip)?;
    match inst {
        Instruction::Terminate => Ok(None),
        Instruction::Add(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            program[i3] = x1 + x2;
            Ok(Some(ip + inst.size()))
        }
        Instruction::Mul(i1, i2, i3) => {
            let x1 = read_parameter(i1, program)?;
            let x2 = read_parameter(i2, program)?;
            program[i3] = x1 * x2;
            Ok(Some(ip + inst.size()))
        }
        Instruction::Input(i1) => {
            let hc_in = 1;
            println!("input asked, giving {}", hc_in);
            program[i1] = hc_in;
            Ok(Some(ip + inst.size()))
        }
        Instruction::Output(i1) => {
            let x1 = read_parameter(i1, program)?;
            println!("output: {}", x1);
            Ok(Some(ip + inst.size()))
        }
    }
}

fn execute_until_termination(program: &mut Vec<usize>) -> Result<usize, &'static str> {
    let mut ip: usize = 0;
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

pub fn part1() -> io::Result<usize> {
    let mut x = read_input()?;
    let result = execute_until_termination(&mut x).unwrap();
    Ok(result)
}

fn execute_with_input(program: &[usize], x1: usize, x2: usize) -> Result<usize, &'static str> {
    let mut prog_copy: Vec<usize> = program.to_vec();
    prog_copy[1] = x1;
    prog_copy[2] = x2;
    execute_until_termination(&mut prog_copy)
}

use clap::{value_t_or_exit, App, Arg};
pub fn part2_manual() -> io::Result<usize> {
    let matches = App::new("run computer")
        .arg(Arg::with_name("x0"))
        .arg(Arg::with_name("x1"))
        .get_matches();
    let x0 = value_t_or_exit!(matches, "x0", usize);
    let x1 = value_t_or_exit!(matches, "x1", usize);

    let x = read_input()?;
    let result = execute_with_input(&x, x0, x1).unwrap();
    Ok(result)
}

pub fn part2() -> io::Result<(usize, usize)> {
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
    let mut program: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
    execute_instruction(&mut program, 0).unwrap();
    assert_eq!(program[5], 9801);
}
