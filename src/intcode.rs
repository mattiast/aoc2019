#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Positional(usize),
    Immediate(isize),
    Relative(isize),
}

#[derive(Debug)]
pub enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
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
        2 => Ok(Parameter::Relative(x)),
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
        let p3 = parse_parameter(program, ip + 3, (val0 / 10000) % 10)?;
        return Ok(Instruction::Add(p1, p2, p3));
    }
    if opcode == 2 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let p3 = parse_parameter(program, ip + 3, (val0 / 10000) % 10)?;
        return Ok(Instruction::Mul(p1, p2, p3));
    }
    if opcode == 3 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
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
        let p3 = parse_parameter(program, ip + 3, (val0 / 10000) % 10)?;
        return Ok(Instruction::LessThan(p1, p2, p3));
    }
    if opcode == 8 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        let p2 = parse_parameter(program, ip + 2, (val0 / 1000) % 10)?;
        let p3 = parse_parameter(program, ip + 3, (val0 / 10000) % 10)?;
        return Ok(Instruction::Equals(p1, p2, p3));
    }
    if opcode == 9 {
        let p1 = parse_parameter(program, ip + 1, (val0 / 100) % 10)?;
        return Ok(Instruction::AdjustRelativeBase(p1));
    }
    Err("invalid opcode")
}

fn read_parameter(p: Parameter, state: &ProgramState) -> Result<isize, &'static str> {
    match p {
        Parameter::Immediate(x) => Ok(x),
        Parameter::Positional(a) => {
            let x = state.mem.get(a).ok_or("out of bounds")?;
            Ok(*x)
        }
        Parameter::Relative(a) => {
            let addr = (state.relative_base as isize + a) as usize;
            let x = state.mem.get(addr).ok_or("out of bounds")?;
            Ok(*x)
        }
    }
}

fn write_to_memory(
    p: Parameter,
    value: isize,
    state: &mut ProgramState,
) -> Result<(), &'static str> {
    let addr = match p {
        Parameter::Immediate(_) => return Err("trying to write to immediate value"),
        Parameter::Positional(a) => a,
        Parameter::Relative(a) => ((state.relative_base as isize + a) as usize),
    };
    state.mem[addr] = value;
    Ok(())
}

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
            let x1 = read_parameter(i1, &state)?;
            let x2 = read_parameter(i2, &state)?;
            write_to_memory(i3, x1 + x2, state)?;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Mul(i1, i2, i3) => {
            let x1 = read_parameter(i1, &state)?;
            let x2 = read_parameter(i2, &state)?;
            write_to_memory(i3, x1 * x2, state)?;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Input(i1) => {
            let inpt = input.ok_or("ran out of inputs")?;
            *input = None;
            write_to_memory(i1, inpt, state)?;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Output(i1) => {
            let x1 = read_parameter(i1, &state)?;
            state.ip += inst.size();
            Ok(Some(x1))
        }
        Instruction::JumpIfTrue(i1, i2) => {
            let x1 = read_parameter(i1, &state)?;
            if x1 != 0 {
                let x2 = read_parameter(i2, &state)?;
                state.ip = x2 as usize;
                Ok(None)
            } else {
                state.ip += inst.size();
                Ok(None)
            }
        }
        Instruction::JumpIfFalse(i1, i2) => {
            let x1 = read_parameter(i1, &state)?;
            if x1 == 0 {
                let x2 = read_parameter(i2, &state)?;
                state.ip = x2 as usize;
                Ok(None)
            } else {
                state.ip += inst.size();
                Ok(None)
            }
        }
        Instruction::LessThan(i1, i2, i3) => {
            let x1 = read_parameter(i1, &state)?;
            let x2 = read_parameter(i2, &state)?;
            let result = if x1 < x2 { 1 } else { 0 };
            write_to_memory(i3, result, state)?;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::Equals(i1, i2, i3) => {
            let x1 = read_parameter(i1, &state)?;
            let x2 = read_parameter(i2, &state)?;
            let result = if x1 == x2 { 1 } else { 0 };
            write_to_memory(i3, result, state)?;
            state.ip += inst.size();
            Ok(None)
        }
        Instruction::AdjustRelativeBase(i1) => {
            let x1 = read_parameter(i1, &state)?;
            let new_value = (state.relative_base as isize + x1) as usize;
            state.relative_base = new_value;
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
