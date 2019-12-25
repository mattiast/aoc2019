use crate::intcode::ProgramState;
use ncurses::{addstr, clear, endwin, getch, getstr, initscr, mv, refresh};
use std::io;

fn get_string(output: Vec<isize>) -> String {
    String::from_utf8(output.iter().map(|c| *c as u8).collect()).unwrap()
}

#[derive(Clone, Copy)]
enum Tile {
    Unknown,
    Room,
}

struct GameState {
    pos: (usize, usize),
    grid: [[Tile; 20]; 20],
}

impl GameState {
    fn init() -> GameState {
        let mut grid = [[Tile::Unknown; 20]; 20];
        let pos = (10, 10);
        grid[pos.0][pos.1] = Tile::Room;
        GameState { pos, grid }
    }

    fn render(&self) -> String {
        let mut s = String::with_capacity(420);

        for (y, line) in self.grid.iter().enumerate().rev() {
            for (x, tile) in line.iter().enumerate() {
                let c = if (x, y) == self.pos {
                    'M'
                } else {
                    match tile {
                        Tile::Unknown => ' ',
                        Tile::Room => '.',
                    }
                };
                s.push(c);
            }
            s.push('\n');
        }

        s
    }

    fn step(&mut self, dx: isize, dy: isize) {
        let (x, y) = self.pos;
        let nx = (x as isize) + dx;
        let ny = (y as isize) + dy;
        self.pos = (nx as usize, ny as usize);

        self.grid[self.pos.0][self.pos.1] = Tile::Room;
    }
}

pub fn part1() -> io::Result<()> {
    initscr();

    let mut ps = ProgramState::init_from_file("data/input25.txt")?;

    let (out, _) = ps.run_with_input(&[]).unwrap();
    addstr(&get_string(out));
    while !ps.terminated {
        /*
        let c = getch();
        println!("{}", c);
        let (line, dir) = match c {
            104 => ("west", (-1, 0)),
            106 => ("south", (0, -1)),
            107 => ("north", (0, 1)),
            108 => ("east", (1, 0)),
            _ => continue,
        };
        */
        let mut line = String::new();
        getstr(&mut line);
        let mut inp: Vec<_> = line.as_bytes().iter().map(|c| *c as isize).collect();
        inp.push(10);
        let (out, _) = ps.run_with_input(&inp).unwrap();
        clear();
        let s = get_string(out);
        addstr(&s);
        // mv(20, 0);
        // addstr(&state.render());
        if ps.terminated {
            endwin();
            println!("{}", s);
        }
    }

    Ok(())
}
