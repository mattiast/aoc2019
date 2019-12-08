#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Positional(usize),
    Immediate(isize),
}

#[derive(Debug)]
pub enum Instruction {
    Add(Parameter, Parameter, usize),
    Mul(Parameter, Parameter, usize),
    Input(usize),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),
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
            Instruction::Terminate => 1,
        }
    }

    pub fn needs_input(&self) -> bool {
        match self {
            Instruction::Input(_) => true,
            _ => false,
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
        return Ok(Instruction::Add(p1, p2, x3 as usize));
    }
    if opcode == 2 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::Mul(p1, p2, x3 as usize));
    }
    if opcode == 3 {
        let x1 = *program.get(ip + 1).ok_or("out of bounds")?;
        return Ok(Instruction::Input(x1 as usize));
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
        return Ok(Instruction::LessThan(p1, p2, x3 as usize));
    }
    if opcode == 8 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let x3 = *program.get(ip + 3).ok_or("out of bounds")?;
        return Ok(Instruction::Equals(p1, p2, x3 as usize));
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

pub struct ProgramState {
    pub mem: Vec<isize>,
    pub ip: usize,
    pub terminated: bool,
}

impl ProgramState {
    pub fn init(code: Vec<isize>) -> ProgramState {
        ProgramState {
            mem: code,
            ip: 0,
            terminated: false,
        }
    }
}

pub fn execute_instruction(
    state: &mut ProgramState,
    inst: Instruction,
    input: &mut Option<isize>,
) -> Result<Option<isize>, &'static str> {
    assert!(!state.terminated);
    match inst {
        Instruction::Terminate => {
            state.terminated = true;
            Ok(None)
        }
        Instruction::Add(i1, i2, i3) => {
            let x1 = read_parameter(i1, &state.mem)?;
            let x2 = read_parameter(i2, &state.mem)?;
            state.mem[i3] = x1 + x2;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Mul(i1, i2, i3) => {
            let x1 = read_parameter(i1, &state.mem)?;
            let x2 = read_parameter(i2, &state.mem)?;
            state.mem[i3] = x1 * x2;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Input(i1) => {
            let inpt = input.ok_or("ran out of inputs")?;
            *input = None;
            state.mem[i1] = inpt;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Output(i1) => {
            let x1 = read_parameter(i1, &state.mem)?;
            state.ip += inst.size();
            Ok(Some(x1))
        }
        Instruction::JumpIfTrue(i1, i2) => {
            let x1 = read_parameter(i1, &state.mem)?;
            if x1 != 0 {
                let x2 = read_parameter(i2, &state.mem)?;
                state.ip = x2 as usize;
                Ok(None)
            } else {
                state.ip += inst.size();
                Ok(None)
            }
        }
        Instruction::JumpIfFalse(i1, i2) => {
            let x1 = read_parameter(i1, &state.mem)?;
            if x1 == 0 {
                let x2 = read_parameter(i2, &state.mem)?;
                state.ip = x2 as usize;
                Ok(None)
            } else {
                state.ip += inst.size();
                Ok(None)
            }
        }
        Instruction::LessThan(i1, i2, i3) => {
            let x1 = read_parameter(i1, &state.mem)?;
            let x2 = read_parameter(i2, &state.mem)?;
            let result = if x1 < x2 { 1 } else { 0 };
            state.mem[i3] = result;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Equals(i1, i2, i3) => {
            let x1 = read_parameter(i1, &state.mem)?;
            let x2 = read_parameter(i2, &state.mem)?;
            let result = if x1 == x2 { 1 } else { 0 };
            state.mem[i3] = result;
            state.ip += inst.size();
            Ok(None)
        }
    }
}

#[test]
fn test_execute_instruction() {
    let mut state = ProgramState::init(vec![2, 4, 4, 5, 99, 0]);
    let inst = parse_instruction(&state.mem, 0).unwrap();
    let mut iter = None;
    execute_instruction(&mut state, inst, &mut iter).unwrap();
    assert_eq!(state.mem[5], 9801);
}
