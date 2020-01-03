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

type Point = (usize, usize);
type Maze = Vec<Vec<Tile>>;

fn read_maze() -> io::Result<State> {
    let file = File::open("data/input18.txt")?;
    // let file = File::open("day18sample.txt")?;
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

fn find_neighbors((x, y): Point, grid: &[Vec<Tile>]) -> Vec<Point> {
    let cands = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    cands
        .into_iter()
        .filter(|p| grid[p.0][p.1].can_step())
        .collect()
}

#[derive(Clone)]
struct State {
    grid: Maze,
    location: Point,
    odometer: usize,
}

type Key = (Point, char, usize);

impl State {
    fn find_reachable_keys(&self) -> Vec<Key> {
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

    fn move_to_key(&mut self, (p, c, d): Key) {
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

use rand::thread_rng;
use rand::Rng;
pub fn part1() -> io::Result<()> {
    artificial_intelligence()?;
    Ok(())
}

fn _random_search() -> io::Result<()> {
    let state = read_maze()?;

    let mut rng = thread_rng();
    let mut best = 10000usize;

    for _ in 0..100 {
        let mut state = state.clone();
        loop {
            let keys = state.find_reachable_keys();
            if keys.is_empty() {
                break;
            }
            let key = keys[rng.gen_range(0, keys.len())];
            state.move_to_key(key);
        }
        best = best.min(state.odometer);
    }
    println!("Best distance {}", best);

    Ok(())
}

use std::cmp::Reverse;
#[derive(Ord, Eq, PartialEq, PartialOrd)]
struct ReachedState(usize, Reverse<usize>, Vec<Key>);

use std::collections::BinaryHeap;

fn artificial_intelligence() -> io::Result<()> {
    let state = read_maze()?;

    let mut reached_states: BinaryHeap<ReachedState> = BinaryHeap::new();

    reached_states.push(ReachedState(0, Reverse(0), Vec::new()));

    let mut best = 5000;
    while let Some(ReachedState(k, Reverse(d), keys)) = reached_states.pop() {
        if k == 26 && d < best {
            best = d;
            println!("Can do in {}", d);
        }
        let mut state = state.clone();
        for key in keys.iter() {
            state.move_to_key(*key);
        }
        let next = state.find_reachable_keys();
        for key in next {
            let mut nkeys = keys.clone();
            nkeys.push(key);
            reached_states.push(ReachedState(k + 1, Reverse(d + key.2), nkeys));
        }
    }

    Ok(())
}
