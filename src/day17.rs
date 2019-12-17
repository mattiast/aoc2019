use crate::intcode::ProgramState;
use std::io;

pub fn part1() -> io::Result<()> {
    let mut ps = ProgramState::init_from_file("data/input17.txt")?;

    let mut x: Vec<u8> = vec![];
    while !ps.terminated {
        let inst = ps.parse_instruction().unwrap();
        let output = ps.execute_instruction(inst, &mut None).unwrap();

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

fn find_crossings(grid: &Vec<Vec<u8>>) {
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

pub fn part2() -> io::Result<()> {
    let mut ps = ProgramState::init_from_file("data/input17.txt")?;
    ps.mem[0] = 2;

    let mut output: Vec<u8> = vec![];
    let mut input = {
        let mut file = File::open("day17code.txt")?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        data
    };

    while !ps.terminated {
        let inst = ps.parse_instruction().unwrap();
        let mut i: Option<isize> = None;
        if inst.needs_input() {
            if input.is_empty() {
                panic!();
            } else {
                i = Some(input.remove(0) as isize);
            }
        }
        let m_outc = ps.execute_instruction(inst, &mut i).unwrap();

        if let Some(outc) = m_outc {
            output.push(outc as u8);
            if outc == 10 {
                let s = String::from_utf8(output).unwrap();
                print!("{}", s);
                output = vec![];
            }
        }
    }
    Ok(())
}
