
#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Positional(usize),
    Immediate(isize),
}

#[derive(Debug)]
pub enum Instruction {
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

pub fn parse_instruction(program: &[isize], ip: usize) -> Result<Instruction, &'static str> {
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

pub enum ER {
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

pub fn execute_instruction<I>(
    program: &mut Vec<isize>,
    ip: isize,
    inst: Instruction,
    inputs: &mut I,
) -> Result<ER, &'static str>
where
    I: Iterator<Item = isize>,
{
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

#[test]
fn test_execute_instruction() {
    let mut program: Vec<isize> = vec![2, 4, 4, 5, 99, 0];
    let inst = parse_instruction(&program, 0).unwrap();
    let mut iter = (vec![]).into_iter();
    execute_instruction(&mut program, 0, inst, &mut iter).unwrap();
    assert_eq!(program[5], 9801);
}