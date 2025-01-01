use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Immediate(isize),
    Location(Address),
}

#[derive(Clone, Copy, Debug)]
pub enum Address {
    Positional(usize),
    Relative(isize),
}

#[derive(Debug)]
pub enum Instruction {
    Add(Parameter, Parameter, Address),
    Mul(Parameter, Parameter, Address),
    Input(Address),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Address),
    Equals(Parameter, Parameter, Address),
    AdjustRelativeBase(Parameter),
    Terminate,
}

impl Instruction {
    fn size(&self) -> usize {
        match self {
            Instruction::Add(_, _, _) => 4,
            Instruction::Mul(_, _, _) => 4,
            Instruction::Input(_) => 2,
            Instruction::Output(_) => 2,
            Instruction::JumpIfTrue(_, _) => 3,
            Instruction::JumpIfFalse(_, _) => 3,
            Instruction::LessThan(_, _, _) => 4,
            Instruction::Equals(_, _, _) => 4,
            Instruction::AdjustRelativeBase(_) => 2,
            Instruction::Terminate => 1,
        }
    }

    pub fn needs_input(&self) -> bool {
        matches!(self, Instruction::Input(_))
    }
}

#[derive(Debug)]
pub enum Error {
    Custom(String),
    InvalidParameterMode,
    InvalidOpCode,
    OutOfBounds,
    CantWriteImmediateValue,
    OutOfInputs,
    ProgramTerminated,
}

fn parse_parameter(program: &[isize], addr: usize, mode_code: usize) -> Result<Parameter, Error> {
    let x = *program.get(addr).ok_or(Error::OutOfBounds)?;
    match mode_code {
        0 => Ok(Parameter::Location(Address::Positional(x as usize))),
        1 => Ok(Parameter::Immediate(x)),
        2 => Ok(Parameter::Location(Address::Relative(x))),
        _ => Err(Error::InvalidParameterMode),
    }
}

fn parse_address(program: &[isize], addr: usize, mode_code: usize) -> Result<Address, Error> {
    let x = *program.get(addr).ok_or(Error::OutOfBounds)?;
    match mode_code {
        0 => Ok(Address::Positional(x as usize)),
        2 => Ok(Address::Relative(x)),
        _ => Err(Error::InvalidParameterMode),
    }
}

#[derive(Clone)]
pub struct ProgramState {
    pub mem: Vec<isize>,
    pub ip: usize,
    pub relative_base: usize,
    pub terminated: bool,
}

impl ProgramState {
    pub fn init(mut code: Vec<isize>) -> ProgramState {
        code.extend(vec![0; 10000]);
        ProgramState {
            mem: code,
            ip: 0,
            relative_base: 0,
            terminated: false,
        }
    }

    pub fn init_from_file(path: &str) -> io::Result<ProgramState> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buf = "".to_owned();
        reader.read_line(&mut buf)?;
        let bb = buf.trim_end();
        let numbers: Vec<_> = bb.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
        let ps = ProgramState::init(numbers);
        Ok(ps)
    }

    pub fn parse_instruction(&self) -> Result<Instruction, Error> {
        let program = &self.mem;
        let ip = self.ip;
        let val0 = *program.get(ip).ok_or(Error::OutOfBounds)? as usize;
        let opcode = val0 % 100;
        if opcode == 99 {
            return Ok(Instruction::Terminate);
        }
        if opcode == 1 {
            let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
            let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
            let p3 = parse_address(program, ip + 3, (val0 / 10000) % 10)?;
            return Ok(Instruction::Add(p1, p2, p3));
        }
        if opcode == 2 {
            let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
            let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
            let p3 = parse_address(program, ip + 3, (val0 / 10000) % 10)?;
            return Ok(Instruction::Mul(p1, p2, p3));
        }
        if opcode == 3 {
            let p1 = parse_address(program, ip + 1, (val0 / 100) % 10)?;
            return Ok(Instruction::Input(p1));
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
            let p3 = parse_address(program, ip + 3, (val0 / 10000) % 10)?;
            return Ok(Instruction::LessThan(p1, p2, p3));
        }
        if opcode == 8 {
            let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
            let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
            let p3 = parse_address(program, ip + 3, (val0 / 10000) % 10)?;
            return Ok(Instruction::Equals(p1, p2, p3));
        }
        if opcode == 9 {
            let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
            return Ok(Instruction::AdjustRelativeBase(p1));
        }
        Err(Error::InvalidOpCode)
    }

    pub fn execute_instruction(
        &mut self,
        inst: Instruction,
        input: &mut Option<isize>,
    ) -> Result<Option<isize>, Error> {
        if self.terminated {
            return Err(Error::ProgramTerminated);
        }
        match inst {
            Instruction::Terminate => {
                self.terminated = true;
                Ok(None)
            }
            Instruction::Add(i1, i2, i3) => {
                let x1 = read_parameter(i1, &self)?;
                let x2 = read_parameter(i2, &self)?;
                write_to_memory(i3, x1 + x2, self)?;
                self.ip += inst.size();
                Ok(None)
            }
            Instruction::Mul(i1, i2, i3) => {
                let x1 = read_parameter(i1, &self)?;
                let x2 = read_parameter(i2, &self)?;
                write_to_memory(i3, x1 * x2, self)?;
                self.ip += inst.size();
                Ok(None)
            }
            Instruction::Input(i1) => {
                let inpt = input.ok_or(Error::OutOfInputs)?;
                *input = None;
                write_to_memory(i1, inpt, self)?;
                self.ip += inst.size();
                Ok(None)
            }
            Instruction::Output(i1) => {
                let x1 = read_parameter(i1, &self)?;
                self.ip += inst.size();
                Ok(Some(x1))
            }
            Instruction::JumpIfTrue(i1, i2) => {
                let x1 = read_parameter(i1, &self)?;
                if x1 != 0 {
                    let x2 = read_parameter(i2, &self)?;
                    self.ip = x2 as usize;
                    Ok(None)
                } else {
                    self.ip += inst.size();
                    Ok(None)
                }
            }
            Instruction::JumpIfFalse(i1, i2) => {
                let x1 = read_parameter(i1, &self)?;
                if x1 == 0 {
                    let x2 = read_parameter(i2, &self)?;
                    self.ip = x2 as usize;
                    Ok(None)
                } else {
                    self.ip += inst.size();
                    Ok(None)
                }
            }
            Instruction::LessThan(i1, i2, i3) => {
                let x1 = read_parameter(i1, &self)?;
                let x2 = read_parameter(i2, &self)?;
                let result = if x1 < x2 { 1 } else { 0 };
                write_to_memory(i3, result, self)?;
                self.ip += inst.size();
                Ok(None)
            }
            Instruction::Equals(i1, i2, i3) => {
                let x1 = read_parameter(i1, &self)?;
                let x2 = read_parameter(i2, &self)?;
                let result = if x1 == x2 { 1 } else { 0 };
                write_to_memory(i3, result, self)?;
                self.ip += inst.size();
                Ok(None)
            }
            Instruction::AdjustRelativeBase(i1) => {
                let x1 = read_parameter(i1, &self)?;
                let new_value = (self.relative_base as isize + x1) as usize;
                self.relative_base = new_value;
                self.ip += inst.size();
                Ok(None)
            }
        }
    }

    pub fn run_with_input(&mut self, input: &[isize]) -> Result<(Vec<isize>, usize), Error> {
        let mut output = Vec::new();
        let mut i = 0usize;

        while !self.terminated {
            let inst = self.parse_instruction()?;
            if inst.needs_input() && i >= input.len() {
                break;
            }
            let mut inp = input.get(i).cloned();
            if inst.needs_input() {
                i += 1;
            }
            let m_out = self.execute_instruction(inst, &mut inp)?;
            if let Some(out) = m_out {
                output.push(out);
            }
        }

        Ok((output, i))
    }
}

fn read_parameter(p: Parameter, state: &ProgramState) -> Result<isize, Error> {
    match p {
        Parameter::Immediate(x) => Ok(x),
        Parameter::Location(a) => read_memory(a, state),
    }
}

fn read_memory(a: Address, state: &ProgramState) -> Result<isize, Error> {
    match a {
        Address::Positional(a) => {
            let x = state.mem.get(a).ok_or(Error::OutOfBounds)?;
            Ok(*x)
        }
        Address::Relative(a) => {
            let addr = (state.relative_base as isize + a) as usize;
            let x = state.mem.get(addr).ok_or(Error::OutOfBounds)?;
            Ok(*x)
        }
    }
}

fn write_to_memory(a: Address, value: isize, state: &mut ProgramState) -> Result<(), Error> {
    let addr = match a {
        Address::Positional(a) => a,
        Address::Relative(a) => (state.relative_base as isize + a) as usize,
    };
    state.mem[addr] = value;
    Ok(())
}

#[test]
fn test_execute_instruction() {
    let mut state = ProgramState::init(vec![2, 4, 4, 5, 99, 0]);
    let inst = state.parse_instruction().unwrap();
    let mut iter = None;
    state.execute_instruction(inst, &mut iter).unwrap();
    assert_eq!(state.mem[5], 9801);
}
