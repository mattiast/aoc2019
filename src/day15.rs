use crate::intcode::ProgramState;
use rand::thread_rng;
use rand::Rng;

fn one_step(ps: &mut ProgramState, direction: isize) -> isize {
    let mut input: Option<isize> = Some(direction);
    let mut output: Option<isize> = None;
    loop {
        match output {
            Some(x) => {
                assert!(input.is_none());
                return x;
            }
            None => {
                let inst = ps.parse_instruction().unwrap();
                output = ps.execute_instruction(inst, &mut input).unwrap();
            }
        }
    }
}

#[derive(Clone)]
enum Tile {
    Unknown,
    Wall,
    Open,
    Oxygen,
}

fn draw_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        let line: String = row.iter()
            .map(|t| match t {
                Tile::Unknown => '?',
                Tile::Wall => '#',
                Tile::Open => ' ',
                Tile::Oxygen => 'O',
            })
            .collect();
        println!("{}", line);
    }
}

fn direction_vec(direction: isize) -> (isize, isize) {
    match direction {
        1 => (0, 1),
        2 => (0, -1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => panic!("wrong directions"),
    }
}

pub fn part1() {
    let mut ps = ProgramState::init_from_file("data/input15.txt").unwrap();
    let mut rng = thread_rng();
    let mut pos: (isize, isize) = (25, 25);
    let mut grid: Vec<Vec<Tile>> = vec![ vec![ Tile::Unknown; 50]; 50];

    for i in 0..1_000_000 {
        let direction: isize = rng.gen_range(1, 5);
        let (dx, dy) = direction_vec(direction);
        let output = one_step(&mut ps, direction);
        if output == 0 {
            grid[(pos.0 + dx) as usize][(pos.1 + dy) as usize] = Tile::Wall;
        }
        if output == 1 {
            pos.0 += dx;
            pos.1 += dy;
            grid[pos.0 as usize][pos.1 as usize] = Tile::Open;
        }
        if output == 2 {
            pos.0 += dx;
            pos.1 += dy;
            grid[pos.0 as usize][pos.1 as usize] = Tile::Oxygen;
        }
    }
    draw_grid(&grid);
}
