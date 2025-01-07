use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    Portal(char, char),
}

impl Tile {
    fn is_portal(self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Open => false,
            Tile::Portal(_, _) => true,
        }
    }
    fn is_open(self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Open => true,
            Tile::Portal(_, _) => false,
        }
    }
}
// How to represent the maze? It will have portals between the inner edge and outer edge.

type Point = (usize, usize);
struct Maze {
    grid: Vec<Vec<Tile>>,
}

fn find_neighbors((x, y): Point, maze: &Maze) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::with_capacity(4);
    let cands = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    for p in cands {
        let tile = maze.grid[p.0][p.1];
        if tile.is_open() {
            result.push(p);
        }
        if let Tile::Portal(_, _) = tile {
            result.push(p);
        }
    }
    result
}

fn resolve_portal(maze: &Maze, p: Point) -> Point {
    let tile = maze.grid[p.0][p.1];
    for (x, row) in maze.grid.iter().enumerate() {
        for (y, tile2) in row.iter().enumerate() {
            if tile == *tile2 && (x, y) != p {
                let cands = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                for p in cands {
                    let tile = maze.grid[p.0][p.1];
                    if tile.is_open() {
                        return p;
                    }
                }
            }
        }
    }
    panic!()
}

pub fn part1() -> io::Result<()> {
    let maze = read_maze()?;
    check_maze(&maze).unwrap();

    for row in maze.grid.iter() {
        for tile in row.iter() {
            if let Tile::Portal(c, d) = tile {
                println!("portal {}{}", c, d);
            }
        }
    }
    let n = find_neighbors((2, 33), &maze);
    println!("{:?}", n);

    let d = bfs(&maze, (106, 73));
    println!("{:?}", d);

    Ok(())
}

fn bfs(maze: &Maze, start: Point) -> usize {
    let mut to_visit: Vec<(Point, usize)> = Vec::with_capacity(100);
    let mut visited: HashSet<Point> = HashSet::new();

    to_visit.push((start, 0));
    while !to_visit.is_empty() {
        let (p, d) = to_visit.remove(0);
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);
        if let Tile::Open = maze.grid[p.0][p.1] {
            let ns = find_neighbors(p, maze);
            for n in ns {
                let tile = maze.grid[n.0][n.1];
                if tile.is_portal() {
                    if tile == Tile::Portal('A', 'A') {
                        continue;
                    }
                    if tile == Tile::Portal('Z', 'Z') {
                        return d;
                    }
                    to_visit.push((resolve_portal(maze, n), d + 1));
                } else {
                    to_visit.push((n, d + 1));
                }
            }
        }
    }
    7
}

fn read_maze() -> io::Result<Maze> {
    let file = File::open("data/input20.txt")?;
    let reader = BufReader::new(file);

    let mut result = vec![];
    for mline in reader.lines() {
        let line = mline?;
        let row: Vec<_> = line
            .chars()
            .map(|c| match c {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                ' ' => Tile::Wall,
                c if c.is_ascii() => Tile::Portal(c, c),
                _ => panic!(),
            })
            .collect();
        result.push(row);
    }

    let m = result.len();
    let n = result[0].len();

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if let Tile::Portal(c, _) = result[i][j] {
                let cands = [(i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)];
                let x = cands.iter().any(|(i1, j1)| result[*i1][*j1].is_open());
                if !x {
                    continue;
                }
                for (i1, j1) in cands.iter() {
                    if let Tile::Portal(d, _) = result[*i1][*j1] {
                        let real_portal = if i + j < i1 + j1 {
                            Tile::Portal(c, d)
                        } else {
                            Tile::Portal(d, c)
                        };
                        result[i][j] = real_portal;
                        result[*i1][*j1] = Tile::Wall;
                    }
                }
            }
        }
    }

    let result = Maze { grid: result };

    Ok(result)
}

fn outer_boundary((i, j): (usize, usize)) -> bool {
    let di = (i as isize - 54).abs();
    let dj = (j as isize - 54).abs();
    return di.max(dj) == 53;
}
fn inner_boundary((i, j): (usize, usize)) -> bool {
    let di = (i as isize - 54).abs();
    let dj = (j as isize - 54).abs();
    return di.max(dj) == 27;
}
fn check_maze(maze: &Maze) -> Result<(), String> {
    let mut portal_locs: HashMap<(char, char), Vec<Point>> = HashMap::new();
    for (i, row) in maze.grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if let Tile::Portal(c, d) = tile {
                portal_locs.entry((*c, *d)).or_default().push((i, j));
            }
        }
    }
    // Check that each portal is in exactly two locations
    for (name, locs) in portal_locs.iter() {
        if locs.len() != 2 {
            if locs.len() == 1 && (name == &('A', 'A') || name == &('Z', 'Z')) {
                // This is OK
                continue;
            } else {
                return Err(format!(
                    "Portal {:?} has {} locations {:?}",
                    name,
                    locs.len(),
                    locs
                ));
            }
        }
        let (p1, p2) = (locs[0], locs[1]);
        // Check that one of the locations is in the inner boundary, and the other is in the outer boundary
        if (inner_boundary(p1) || inner_boundary(p2)) && (outer_boundary(p1) || outer_boundary(p2))
        {
            // This is OK
        } else {
            return Err(format!(
                "Portal {:?} has locations {:?} and {:?}",
                name, p1, p2
            ));
        }
    }

    Ok(())
}
