use crate::intcode::ProgramState;
use std::io;
pub fn part1() -> io::Result<()> {
    let ps = ProgramState::init_from_file("data/input19.txt")?;

    let mut total = 0;
    for x in 0..50 {
        for y in 0..50 {
            total += run_with_input(ps.clone(), x, y);
        }
    }
    println!("Total: {}", total);

    Ok(())
}

use clap::{value_t_or_exit, App, Arg};
pub fn part2() -> io::Result<()> {
    let matches = App::new("run computer")
        .arg(Arg::with_name("x"))
        .arg(Arg::with_name("y"))
        .get_matches();
    let x = value_t_or_exit!(matches, "x", isize);
    let y = value_t_or_exit!(matches, "y", isize);
    let ps = ProgramState::init_from_file("data/input19.txt")?;

    println!("x lo {}", run_with_input(ps.clone(), x + 99, y));
    println!("x hi {}", run_with_input(ps.clone(), x + 100, y));
    println!("y lo {}", run_with_input(ps.clone(), x, y + 99));
    println!("y hi {}", run_with_input(ps.clone(), x, y + 100));

    Ok(())
}

fn run_with_input(mut ps: ProgramState, x: isize, y: isize) -> isize {
    let mut input = vec![x, y];
    let mut output = vec![];

    while !ps.terminated {
        let inst = ps.parse_instruction().unwrap();
        let mut i = if inst.needs_input() {
            Some(input.remove(0))
        } else {
            None
        };
        let m_out = ps.execute_instruction(inst, &mut i).unwrap();
        if let Some(out) = m_out {
            output.push(out);
        }
    }
    assert!(output.len() == 1);
    output[0]
}