use crate::intcode::ProgramState;

use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Vec<isize>> {
    let file = File::open("data/input11.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let numbers: Vec<_> = bb.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
    Ok(numbers)
}

fn run_round(input: bool, ps: &mut ProgramState) -> Result<(bool, bool), &'static str> {
    let mut inp = Some(if input { 1 } else { 0 });
    let mut output: Vec<isize> = vec![];
    loop {
        let inst = ps.parse_instruction()?;
        let out = ps.execute_instruction(inst, &mut inp)?;
        if let Some(x) = out {
            output.push(x);
        }
        if output.len() >= 2 {
            break;
        }
    }
    loop {
        let inst = ps.parse_instruction()?;
        if inst.needs_input() || ps.terminated {
            break;
        }
        let out = ps.execute_instruction(inst, &mut inp)?;
        assert!(out.is_none());
    }
    assert!(inp.is_none());
    if output[0] < 0 || output[0] > 1 || output[1] < 0 || output[1] > 1 {
        return Err("invalid output");
    }

    Ok((output[0] == 1, output[1] == 1))
}

use std::collections::HashSet;
pub fn part1() -> io::Result<()> {
    let input = read_input()?;

    let mut ps = ProgramState::init(input);

    let mut grid: Vec<Vec<bool>> = vec![vec![true; 100]; 100];
    let mut pos: (usize, usize) = (50, 50);
    let mut dir: (isize, isize) = (0, 1);
    let mut painted_points: HashSet<(usize, usize)> = HashSet::new();

    while !ps.terminated {
        let inp = grid[pos.0][pos.1];
        let (color, turn) = run_round(inp, &mut ps).unwrap();
        grid[pos.0][pos.1] = color;
        painted_points.insert(pos);
        if turn {
            dir = (dir.1, -dir.0);
        } else {
            dir = (-dir.1, dir.0);
        }
        pos.0 = (pos.0 as isize + dir.0) as usize;
        pos.1 = (pos.1 as isize + dir.1) as usize;
    }
    println!("{:?}", painted_points.len());
    println!(
        "{:?} {:?} {:?} {:?}",
        painted_points.iter().map(|p| p.0).min(),
        painted_points.iter().map(|p| p.0).max(),
        painted_points.iter().map(|p| p.1).min(),
        painted_points.iter().map(|p| p.1).max(),
    );
    for j in (44..51).rev() {
        let line: String = (45..100)
            .map(|i| if grid[i][j] { '#' } else { ' ' })
            .collect();
        println!("{}", line);
    }

    Ok(())
}
