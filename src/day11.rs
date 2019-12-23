use crate::intcode::ProgramState;
use std::collections::HashSet;
use std::io;

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

pub fn part1() -> io::Result<()> {
    let mut ps = ProgramState::init_from_file("data/input11.txt")?;

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
    let x_min = painted_points.iter().map(|p| p.0).min().unwrap();
    let x_max = painted_points.iter().map(|p| p.0).max().unwrap();
    let y_min = painted_points.iter().map(|p| p.1).min().unwrap();
    let y_max = painted_points.iter().map(|p| p.1).max().unwrap();
    for j in (y_min..=y_max).rev() {
        let line: String = (x_min..=x_max)
            .map(|i| if grid[i][j] { '#' } else { ' ' })
            .collect();
        println!("{}", line);
    }

    Ok(())
}
