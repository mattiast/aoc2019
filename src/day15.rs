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
}

#[derive(Clone, Copy)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

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
        let i: isize = rng.gen_range(1, 5);
        match i {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!(),
        }
    }
}

fn draw_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        let line: String = row
            .iter()
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

fn artificial_intelligence<T>(
    (x, y): (usize, usize),
    grid: &Vec<Vec<Tile>>,
    rng: &mut T,
) -> Direction
where
    T: Rng,
{
    if grid[x][y + 1] == Tile::Unknown {
        return Direction::North;
    }
    if grid[x][y - 1] == Tile::Unknown {
        return Direction::South;
    }
    if grid[x - 1][y] == Tile::Unknown {
        return Direction::West;
    }
    if grid[x + 1][y] == Tile::Unknown {
        return Direction::East;
    }
    Direction::random(rng)
}

pub fn part1() {
    let mut ps = ProgramState::init_from_file("data/input15.txt").unwrap();
    let mut rng = thread_rng();
    let mut pos: (usize, usize) = (25, 25);
    let mut dir: Direction = Direction::North;
    let mut grid: Vec<Vec<Tile>> = vec![vec![Tile::Unknown; 50]; 50];

    let mut turned: bool = false;
    for _ in 0..100_000 {
        //let direction = artificial_intelligence(pos, &grid, &mut rng);
        let direction = dir.left();
        let next_pos = direction.step(pos);
        let output = one_step(&mut ps, direction.code());
        let tile = Tile::from_code(output);
        grid[next_pos.0][next_pos.1] = tile;
        if tile != Tile::Wall {
            pos = next_pos;
        }
    }
    draw_grid(&grid);
}
