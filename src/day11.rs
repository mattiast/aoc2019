use crate::intcode::ProgramState;
use std::collections::HashSet;
use std::io;

fn run_robot(
    ps: &mut ProgramState,
    grid: &mut Vec<Vec<bool>>,
    painted_points: &mut HashSet<(usize, usize)>,
    pos: &mut (usize, usize),
    dir: &mut (isize, isize),
) {
    while !ps.terminated {
        let inp = grid[pos.0][pos.1];
        let (outs, k) = ps.run_with_input(&[inp as isize]).unwrap();
        assert!(outs.len() == 2);
        assert!(k == 1);
        let (color, turn) = (outs[0] == 1, outs[1] == 1);
        grid[pos.0][pos.1] = color;
        painted_points.insert(*pos);
        if turn {
            *dir = (dir.1, -dir.0);
        } else {
            *dir = (-dir.1, dir.0);
        }
        pos.0 = (pos.0 as isize + dir.0) as usize;
        pos.1 = (pos.1 as isize + dir.1) as usize;
    }
}

pub fn part1() -> io::Result<usize> {
    let mut ps = ProgramState::init_from_file("data/input11.txt")?;

    let mut grid: Vec<Vec<bool>> = vec![vec![false; 100]; 100];
    let mut pos: (usize, usize) = (50, 50);
    let mut dir: (isize, isize) = (0, 1);
    let mut painted_points: HashSet<(usize, usize)> = HashSet::new();

    run_robot(&mut ps, &mut grid, &mut painted_points, &mut pos, &mut dir);

    Ok(painted_points.len())
}

pub fn part2() -> io::Result<()> {
    let mut ps = ProgramState::init_from_file("data/input11.txt")?;

    let mut grid: Vec<Vec<bool>> = vec![vec![true; 100]; 100];
    let mut pos: (usize, usize) = (50, 50);
    let mut dir: (isize, isize) = (0, 1);
    let mut painted_points: HashSet<(usize, usize)> = HashSet::new();

    run_robot(&mut ps, &mut grid, &mut painted_points, &mut pos, &mut dir);
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
