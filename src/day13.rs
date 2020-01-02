use crate::intcode::{self, ProgramState};
use crate::my_error::MyResult;

use ncurses::{addstr, clear, endwin, initscr, mv, refresh};
use std::cmp::Ordering;
use std::io;

fn run_round(ps: &mut ProgramState) -> Result<(isize, isize, isize), intcode::Error> {
    let mut output: Vec<isize> = vec![];
    loop {
        let inst = ps.parse_instruction()?;
        if inst.needs_input() {}
        let out = ps.execute_instruction(inst, &mut None)?;
        if let Some(x) = out {
            output.push(x);
        }
        if output.len() >= 3 {
            break;
        }
    }

    Ok((output[0], output[1], output[2]))
}

fn draw_grid(grid: &Vec<Vec<isize>>, score: isize) {
    for j in 0..26 {
        let line: String = (0..43)
            .map(|i| match grid[i][j] {
                0 => ' ',
                1 => '#',
                2 => 'X',
                3 => '=',
                4 => 'O',
                _ => panic!("wrong tile"),
            })
            .collect();
        mv(j as i32, 0);
        addstr(&line);
    }
    mv(26, 0);
    addstr(&format!("Score {}", score));
}

fn artificial_intelligence(grid: &Vec<Vec<isize>>) -> isize {
    let mut paddle_x: Option<usize> = None;
    let mut ball_x: Option<usize> = None;
    for (i, row) in grid.iter().enumerate() {
        for tile in row.iter() {
            if *tile == 3 {
                if paddle_x.is_some() {
                    panic!("Two paddles");
                }
                paddle_x = Some(i);
            }
            if *tile == 4 {
                if ball_x.is_some() {
                    panic!("Two balls");
                }
                ball_x = Some(i);
            }
        }
    }
    let a = paddle_x.unwrap();
    let b = ball_x.unwrap();
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

pub fn part2() -> MyResult<()> {
    initscr();

    let mut ps = ProgramState::init_from_file("data/input13.txt")?;
    ps.mem[0] = 2;
    let mut grid: Vec<Vec<isize>> = vec![vec![0; 50]; 50];
    let mut score: isize = 0;

    while !ps.terminated {
        let mut output: Vec<isize> = vec![];
        loop {
            let inst = ps.parse_instruction()?;
            let mut i: Option<isize> = if inst.needs_input() {
                clear();
                draw_grid(&grid, score);
                refresh();
                Some(artificial_intelligence(&grid))
            } else {
                None
            };
            let out = ps.execute_instruction(inst, &mut i)?;
            if ps.terminated {
                break;
            }
            if let Some(x) = out {
                output.push(x);
            }
            if output.len() >= 3 {
                break;
            }
        }
        if output.len() < 3 {
            break;
        }

        let out = (output[0], output[1], output[2]);
        if (out.0, out.1) == (-1, 0) {
            score = out.2;
        } else {
            grid[out.0 as usize][out.1 as usize] = out.2;
        }
    }

    endwin();
    println!("GAME OVER! score {}", score);
    Ok(())
}

pub fn part1() -> MyResult<()> {
    let mut ps = ProgramState::init_from_file("data/input13.txt")?;

    let mut grid: Vec<Vec<bool>> = vec![vec![false; 100]; 100];

    while !ps.terminated {
        let out = run_round(&mut ps);
        match out {
            Err(intcode::Error::ProgramTerminated) => {
                break;
            }
            Ok(out) => {
                grid[out.0 as usize][out.1 as usize] = out.2 == 2;
            }
            _ => panic!("weird"),
        }
    }

    for j in (0..26).rev() {
        let line: String = (0..43)
            .map(|i| if grid[i][j] { '#' } else { ' ' })
            .collect();
        println!("{}", line);
    }

    let mut total = 0;
    for row in grid.iter() {
        for x in row.iter() {
            total += if *x { 1 } else { 0 };
        }
    }
    println!("total {}", total);

    Ok(())
}
