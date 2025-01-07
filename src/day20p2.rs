use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

type Point = (usize, usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    PortalIn(Point),
    PortalOut(Point),
}
pub struct Maze {
    grid: Vec<Vec<Tile>>,
    start: Point,
    end: Point,
}

// How to represent the maze? It will have portals between the inner edge and outer edge.
type Position = (Point, usize);

fn find_neighbors(((x, y), d): Position, maze: &Maze) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::with_capacity(4);
    let cands = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    for p in cands {
        let tile = maze.grid[p.0][p.1];
        match tile {
            Tile::Wall => {}
            Tile::Open => {
                result.push((p, d));
            }
            Tile::PortalIn(p2) => {
                result.push((go_from_portal(p2), d + 1));
            }
            Tile::PortalOut(p2) => {
                if d > 0 {
                    result.push((go_from_portal(p2), d - 1));
                }
            }
        }
    }
    result
}

pub fn part1() -> io::Result<()> {
    let maze = parse::read_maze()?;
    let maze = check_maze(&maze).unwrap();

    let d = bfs(&maze);
    println!("{:?}", d);

    Ok(())
}

fn bfs(maze: &Maze) -> usize {
    let mut to_visit: VecDeque<(Position, usize)> = VecDeque::with_capacity(100);
    let mut visited: HashSet<Position> = HashSet::new();

    to_visit.push_back(((maze.start, 0), 0));
    while let Some((p, dist)) = to_visit.pop_front() {
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);
        let ns = find_neighbors(p, maze);
        for n in ns {
            assert_eq!(maze.grid[n.0 .0][n.0 .1], Tile::Open);
            if n == ((maze.end, 0)) {
                return dist + 1;
            }
            to_visit.push_back((n, dist + 1));
        }
    }
    panic!("No path found");
}

fn go_from_portal((mut i, mut j): Point) -> Point {
    if i == 81 {
        i += 1;
    } else if i == 27 {
        i -= 1;
    } else if i == 107 {
        i -= 1;
    } else if i == 1 {
        i += 1;
    } else if j == 81 {
        j += 1;
    } else if j == 27 {
        j -= 1;
    } else if j == 107 {
        j -= 1;
    } else if j == 1 {
        j += 1;
    } else {
        panic!("Unexpected portal location: {:?}", (i, j));
    }
    (i, j)
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
fn check_maze(maze: &parse::Maze) -> Result<Maze, String> {
    let mut portal_locs: HashMap<(char, char), Vec<Point>> = HashMap::new();
    for (i, row) in maze.grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if let parse::Tile::Portal(c, d) = tile {
                portal_locs.entry((*c, *d)).or_default().push((i, j));
            }
        }
    }
    let mut start = None;
    let mut end = None;
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
    let mut grid = Vec::new();
    for (i, row) in maze.grid.iter().enumerate() {
        let mut row2 = Vec::new();
        for (j, tile) in row.iter().enumerate() {
            match tile {
                parse::Tile::Wall => row2.push(Tile::Wall),
                parse::Tile::Open => row2.push(Tile::Open),
                parse::Tile::Portal(c, d) => {
                    if (c, d) == (&'A', &'A') {
                        start = Some((i, j));
                        row2.push(Tile::Open);
                    } else if (c, d) == (&'Z', &'Z') {
                        end = Some((i, j));
                        row2.push(Tile::Open);
                    } else {
                        let locs = portal_locs.get(&(*c, *d)).unwrap();
                        let p = if locs[0] == (i, j) { locs[1] } else { locs[0] };
                        if inner_boundary((i, j)) {
                            row2.push(Tile::PortalIn(p));
                        } else {
                            row2.push(Tile::PortalOut(p));
                        }
                    }
                }
            }
        }
        grid.push(row2);
    }

    Ok(Maze {
        grid,
        start: start.ok_or_else(|| "No start".to_string())?,
        end: end.ok_or_else(|| "No start".to_string())?,
    })
}

mod parse {
    use std::fs::File;
    use std::io::{self, prelude::BufRead, BufReader};

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Tile {
        Wall,
        Open,
        Portal(char, char),
    }
    impl Tile {
        fn is_open(self) -> bool {
            match self {
                Tile::Wall => false,
                Tile::Open => true,
                Tile::Portal(_, _) => false,
            }
        }
    }
    pub struct Maze {
        pub grid: Vec<Vec<Tile>>,
    }
    pub fn read_maze() -> io::Result<Maze> {
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
}
