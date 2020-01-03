use crate::intcode::ProgramState;
use crate::my_error::MyResult;
use std::io;

pub fn part1() -> MyResult<()> {
    let mut ps = ProgramState::init_from_file("data/input17.txt")?;

    let mut x: Vec<u8> = vec![];
    while !ps.terminated {
        let inst = ps.parse_instruction()?;
        let output = ps.execute_instruction(inst, &mut None)?;

        if let Some(out) = output {
            x.push(out as u8);
        }
    }
    let grid: Vec<Vec<u8>> = {
        let mut grid = vec![];
        for line in x.split(|c| *c == 10) {
            if !line.is_empty() {
                grid.push(line.to_vec());
            }
        }
        grid
    };
    find_crossings(&grid);
    Ok(())
}

fn find_crossings(grid: &[Vec<u8>]) {
    let n = grid.len();
    let m = grid[0].len();

    let mut total: u32 = 0;
    for x in 1..n - 1 {
        for y in 1..m - 1 {
            let crossing = (grid[x][y] == 35)
                && (grid[x - 1][y] == 35)
                && (grid[x + 1][y] == 35)
                && (grid[x][y - 1] == 35)
                && (grid[x][y + 1] == 35);
            if crossing {
                println!("crossing: {} {}", x, y);
                total += (x as u32) * (y as u32);
            }
        }
    }
    println!("total {}", total);
}

use std::fs::File;
use std::io::Read;

pub fn part2() -> MyResult<()> {
    let mut ps = ProgramState::init_from_file("data/input17.txt")?;
    ps.mem[0] = 2;

    let mut output: Vec<isize> = vec![];
    let mut input = {
        let mut file = File::open("data/day17code.txt")?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        data
    };

    while !ps.terminated {
        let inst = ps.parse_instruction()?;
        let mut i: Option<isize> = None;
        if inst.needs_input() {
            if input.is_empty() {
                panic!();
            } else {
                i = Some(input.remove(0) as isize);
            }
        }
        let m_outc = ps.execute_instruction(inst, &mut i)?;

        if let Some(outc) = m_outc {
            output.push(outc);
            if outc == 10 {
                let s = String::from_utf8(output.iter().map(|c| *c as u8).collect()).unwrap();
                print!("{}", s);
                output = vec![];
            }
        }
    }
    println!("final output {:?}", output.last());
    Ok(())
}

use std::io::BufRead;
use std::io::BufReader;
fn read_maze() -> io::Result<Vec<Vec<bool>>> {
    let file = File::open("data/day17maze.txt")?;
    let reader = BufReader::new(file);

    let mut result = vec![];
    for mline in reader.lines() {
        let line = mline?;
        let row: Vec<_> = line.chars().map(|c| c == '#').collect();
        result.push(row);
    }

    Ok(result)
}

fn turn((x, y): (usize, usize), (dx, dy): (isize, isize)) -> (usize, usize) {
    let nx = (x as isize) + dx;
    let ny = (y as isize) + dy;
    (nx as usize, ny as usize)
}

fn read(maze: &[Vec<bool>], p: (usize, usize)) -> bool {
    if p.0 < maze.len() {
        let row = &maze[p.0];
        if p.1 < row.len() {
            row[p.1]
        } else {
            false
        }
    } else {
        false
    }
}

pub fn find_code() -> io::Result<()> {
    let maze = read_maze()?;
    let mut pos = (16usize, 0usize);
    let mut dir = (-1isize, 0isize);

    let mut d = 0;
    loop {
        let s = turn(pos, dir);
        let l = turn(pos, (-dir.1, dir.0));
        let r = turn(pos, (dir.1, -dir.0));
        if read(&maze, s) {
            pos = s;
            d += 1;
        } else if read(&maze, l) {
            print!("{}", d);
            d = 0;
            print!("L");
            dir = (-dir.1, dir.0);
        } else if read(&maze, r) {
            print!("{}", d);
            d = 0;
            print!("R");
            dir = (dir.1, -dir.0);
        } else {
            print!("{}", d);
            break;
        }
    }
    println!();

    Ok(())
}
