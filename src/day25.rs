use crate::intcode::ProgramState;
use ncurses::{addstr, clear, endwin, getstr, initscr};
use std::io;

fn get_string(output: Vec<isize>) -> String {
    String::from_utf8(output.iter().map(|c| *c as u8).collect()).unwrap()
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
