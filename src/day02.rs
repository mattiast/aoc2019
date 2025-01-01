use clap::{arg, command};

use crate::intcode::ProgramState;
use std::io;

pub fn part1() -> io::Result<isize> {
    let mut ps = ProgramState::init_from_file("data/input02.txt")?;
    ps.mem[1] = 12;
    ps.mem[2] = 2;
    ps.run_with_input(&[]).unwrap();
    assert!(ps.terminated);
    Ok(ps.mem[0])
}

pub fn part2_manual() -> io::Result<isize> {
    let matches = command!().arg(arg!("x0")).arg(arg!("x1")).get_matches();
    let x0 = *matches.get_one::<isize>("x0").unwrap();
    let x1 = *matches.get_one::<isize>("x1").unwrap();

    let mut ps = ProgramState::init_from_file("data/input02.txt")?;
    ps.mem[1] = x0;
    ps.mem[2] = x1;
    ps.run_with_input(&[]).unwrap();
    assert!(ps.terminated);
    Ok(ps.mem[0])
}

pub fn part2() -> io::Result<(isize, isize)> {
    let ps = ProgramState::init_from_file("data/input02.txt")?;

    for x1 in 0..100 {
        for x2 in 0..100 {
            let mut ps = ps.clone();
            ps.mem[1] = x1;
            ps.mem[2] = x2;
            ps.run_with_input(&[]).unwrap();
            assert!(ps.terminated);
            let result = ps.mem[0];
            if result == 1969_0720 {
                return Ok((x1, x2));
            }
        }
    }
    panic!();
}
