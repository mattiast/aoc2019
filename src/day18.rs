use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    Key(char),
    Door(char),
}

impl Tile {
    fn can_step(self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Open => true,
            Tile::Key(_) => true,
            Tile::Door(_) => false,
        }
    }
}

fn draw_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        let line: String = row
            .iter()
            .map(|t| match t {
                Tile::Wall => '#',
                Tile::Open => ' ',
                Tile::Key(c) => *c,
                Tile::Door(c) => c.to_ascii_uppercase(),
            })
            .collect();
        println!("{}", line);
    }
}

fn read_maze() -> io::Result<State> {
    let file = File::open("data/input18.txt")?;
    let reader = BufReader::new(file);

    let mut entry: Option<Point> = None;

    let mut result = vec![];
    for (x, mline) in reader.lines().enumerate() {
        let line = mline?;
        let row: Vec<_> = line
            .chars()
            .enumerate()
            .map(|(y, c)| match c {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                '@' => {
                    entry = Some((x, y));
                    Tile::Open
                }
                c if c.is_ascii() => {
                    if c.is_lowercase() {
                        Tile::Key(c)
                    } else {
                        Tile::Door(c.to_ascii_lowercase())
                    }
                }
                _ => panic!(),
            })
            .collect();
        result.push(row);
    }
    let pos = entry.unwrap();

    Ok(State {
        grid: result,
        location: pos,
        odometer: 0,
    })
}

fn find_neighbors((x, y): Point, grid: &Maze) -> Vec<Point> {
    let cands = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    cands
        .into_iter()
        .filter(|p| grid[p.0][p.1].can_step())
        .collect()
}

type Point = (usize, usize);
type Maze = Vec<Vec<Tile>>;

struct State {
    grid: Maze,
    location: Point,
    odometer: usize,
}

impl State {
    fn find_reachable_keys(&self) -> Vec<(Point, char, usize)> {
        let mut to_visit: Vec<(Point, usize)> = Vec::with_capacity(100);
        let mut visited: HashSet<Point> = HashSet::new();

        to_visit.push((self.location, 0));
        let mut keys_found = vec![];
        while !to_visit.is_empty() {
            let (p, d) = to_visit.remove(0);
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);
            match self.grid[p.0][p.1] {
                Tile::Key(c) => {
                    keys_found.push((p, c, d));
                }
                Tile::Open => {
                    let ns = find_neighbors(p, &self.grid);
                    for n in ns {
                        to_visit.push((n, d + 1));
                    }
                }
                _ => {}
            }
        }
        keys_found
    }

    fn move_to_key(&mut self, (p, c, d): (Point, char, usize)) {
        self.location = p;
        self.odometer += d;

        self.grid[p.0][p.1] = Tile::Open;
        for row in self.grid.iter_mut() {
            for tile in row.iter_mut() {
                if *tile == Tile::Door(c) {
                    *tile = Tile::Open;
                }
            }
        }
    }
}

pub fn part1() -> io::Result<()> {
    let mut state = read_maze()?;

    loop {
        let keys = state.find_reachable_keys();
        if keys.is_empty() {
            break;
        }
        let key = keys[0];
        state.move_to_key(key);
        println!("moving to key {} out of {}", key.1, keys.len());
    }
    println!("Total distance {}", state.odometer);

    Ok(())
}

fn find_oxygen(grid: &Maze) -> Point {
    for (x, row) in grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if *tile == Tile::Door('a') {
                return (x, y);
            }
        }
    }
    panic!("no oxygen found")
}
