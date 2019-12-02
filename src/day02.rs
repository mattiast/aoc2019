use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};
pub fn read_input() -> io::Result<Vec<usize>> {
    let file = File::open("data/input02.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let numbers: Vec<_> = bb.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
    Ok(numbers)
}

enum Op {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Terminate,
}

fn parse_instruction(program: &[usize], ip: usize) -> Result<Op, &'static str> {
    let opcode = *program.get(ip).ok_or("out of bounds")?;
    if opcode == 99 {
        return Ok(Op::Terminate);
    }
    if opcode == 1 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        let x2 = *program.get(ip + 2).ok_or("out of bounds")?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Op::Add(x1, x2, x3));
    }
    if opcode == 2 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        let x2 = *program.get(ip + 2).ok_or("out of bounds")?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Op::Mul(x1, x2, x3));
    }
    Err("invalid opcode")
}

fn execute_instruction(
    program: &mut Vec<usize>,
    ip: usize,
) -> Result<Option<usize>, &'static str> {
    let inst = parse_instruction(&program, ip)?;
    match inst {
        Op::Terminate => Ok(None),
        Op::Add(i1, i2, i3) => {
            let x1 = program.get(i1).ok_or("out of bounds")?;
            let x2 = program.get(i2).ok_or("out of bounds")?;
            program[i3] = x1 + x2;
            Ok(Some(ip + 4))
        }
        Op::Mul(i1, i2, i3) => {
            let x1 = program.get(i1).ok_or("out of bounds")?;
            let x2 = program.get(i2).ok_or("out of bounds")?;
            program[i3] = x1 * x2;
            Ok(Some(ip + 4))
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
    let x = read_input()?;
    let result = execute_with_input(&x, 12, 2).unwrap();
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
