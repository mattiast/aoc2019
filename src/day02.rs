use crate::intcode::ProgramState;
use clap::{value_t_or_exit, App, Arg};
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
    let matches = App::new("run computer")
        .arg(Arg::with_name("x0"))
        .arg(Arg::with_name("x1"))
        .get_matches();
    let x0 = value_t_or_exit!(matches, "x0", isize);
    let x1 = value_t_or_exit!(matches, "x1", isize);

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
