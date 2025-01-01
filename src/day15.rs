use crate::intcode::ProgramState;
use crate::my_error::MyResult;
use rand::thread_rng;
use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn one_step(ps: &mut ProgramState, direction: isize) -> MyResult<isize> {
    let (output, k) = ps.run_with_input(&[direction])?;
    assert!(output.len() == 1);
    assert!(k == 1);
    Ok(output[0])
}

type Point = (usize, usize);
struct Maze {
    grid: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Wall,
    Open,
    Oxygen,
}

impl Tile {
    fn from_code(code: isize) -> Tile {
        match code {
            0 => Tile::Wall,
            1 => Tile::Open,
            2 => Tile::Oxygen,
            _ => panic!("invalid tile code"),
        }
    }

    fn can_step(self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Open => true,
            Tile::Oxygen => true,
            Tile::Unknown => panic!("dunno if can step"),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn code(self) -> isize {
        self as isize
    }

    fn step(self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }

    fn random<T>(rng: &mut T) -> Direction
    where
        T: Rng,
    {
        let i: isize = rng.gen_range(1..5);
        match i {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!(),
        }
    }
}

fn draw_grid(maze: &Maze) {
    for row in &maze.grid {
        let line: String = row
            .iter()
            .map(|t| match t {
                Tile::Unknown => ' ',
                Tile::Wall => '#',
                Tile::Open => '.',
                Tile::Oxygen => 'O',
            })
            .collect();
        println!("{}", line);
    }
}

fn artificial_intelligence<T>((x, y): (usize, usize), maze: &Maze, rng: &mut T) -> Direction
where
    T: Rng,
{
    if maze.grid[x][y + 1] == Tile::Unknown {
        return Direction::North;
    }
    if maze.grid[x][y - 1] == Tile::Unknown {
        return Direction::South;
    }
    if maze.grid[x - 1][y] == Tile::Unknown {
        return Direction::West;
    }
    if maze.grid[x + 1][y] == Tile::Unknown {
        return Direction::East;
    }
    Direction::random(rng)
}

pub fn print_maze() -> MyResult<()> {
    let mut ps = ProgramState::init_from_file("data/input15.txt")?;
    let mut rng = thread_rng();
    let mut pos: (usize, usize) = (25, 25);
    let mut grid: Vec<Vec<Tile>> = vec![vec![Tile::Unknown; 50]; 50];

    grid[pos.0][pos.1] = Tile::Open; // start from open tile
    let mut maze = Maze { grid };

    for _ in 0..1_000_000 {
        let direction = artificial_intelligence(pos, &maze, &mut rng);
        //let direction = dir.left();
        let next_pos = direction.step(pos);
        let output = one_step(&mut ps, direction.code())?;
        let tile = Tile::from_code(output);
        maze.grid[next_pos.0][next_pos.1] = tile;
        if tile != Tile::Wall {
            pos = next_pos;
        }
    }
    draw_grid(&maze);
    Ok(())
}

fn read_maze() -> io::Result<Maze> {
    let file = File::open("maze.txt")?;
    let reader = BufReader::new(file);

    let mut result = vec![];
    for mline in reader.lines() {
        let line = mline?;
        let row: Vec<_> = line
            .chars()
            .map(|c| match c {
                ' ' => Tile::Unknown,
                '.' => Tile::Open,
                '#' => Tile::Wall,
                'O' => Tile::Oxygen,
                _ => panic!("tile char wrong"),
            })
            .collect();
        result.push(row);
    }

    let result = Maze { grid: result };

    Ok(result)
}

fn find_neighbors((x, y): Point, maze: &Maze) -> Vec<Point> {
    let cands = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    cands
        .into_iter()
        .filter(|p| maze.grid[p.0][p.1].can_step())
        .collect()
}

pub fn part1() -> io::Result<()> {
    let maze = read_maze()?;

    let start: Point = (25, 25);
    let mut to_visit: Vec<(Point, usize)> = Vec::with_capacity(100);
    let mut visited: HashSet<Point> = HashSet::new();

    to_visit.push((start, 0));
    while !to_visit.is_empty() {
        let (p, d) = to_visit.remove(0);
        if visited.contains(&p) {
            continue;
        }
        if maze.grid[p.0][p.1] == Tile::Oxygen {
            println!("Found oxygen at {:?} distance {}", p, d);
            break;
        }
        let ns = find_neighbors(p, &maze);
        for n in ns {
            to_visit.push((n, d + 1));
        }
        visited.insert(p);
    }

    Ok(())
}

fn find_oxygen(maze: &Maze) -> Point {
    for (x, row) in maze.grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if *tile == Tile::Oxygen {
                return (x, y);
            }
        }
    }
    panic!("no oxygen found")
}

pub fn part2() -> io::Result<()> {
    let grid = read_maze()?;

    let start = find_oxygen(&grid);
    let mut to_visit: Vec<(Point, usize)> = Vec::with_capacity(100);
    let mut visited: HashSet<Point> = HashSet::new();

    to_visit.push((start, 0));
    while !to_visit.is_empty() {
        let (p, d) = to_visit.remove(0);
        if visited.contains(&p) {
            continue;
        }
        println!("visiting {:?} distance {}", p, d);
        let ns = find_neighbors(p, &grid);
        for n in ns {
            to_visit.push((n, d + 1));
        }
        visited.insert(p);
    }

    Ok(())
}
