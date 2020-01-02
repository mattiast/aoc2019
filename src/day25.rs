use crate::intcode::ProgramState;
use crate::my_error::MyResult;
use ncurses::{addstr, clear, endwin, getstr, initscr};

fn get_string(output: Vec<isize>) -> String {
    String::from_utf8(output.iter().map(|c| *c as u8).collect()).unwrap()
}

pub fn part1() -> MyResult<()> {
    initscr();

    let mut ps = ProgramState::init_from_file("data/input25.txt")?;

    let (out, _) = ps.run_with_input(&[])?;
    addstr(&get_string(out));
    while !ps.terminated {
        let mut line = String::new();
        getstr(&mut line);
        let mut inp: Vec<_> = line.as_bytes().iter().map(|c| *c as isize).collect();
        inp.push(10);
        let (out, _) = ps.run_with_input(&inp)?;
        clear();
        let s = get_string(out);
        addstr(&s);
        if ps.terminated {
            endwin();
            println!("{}", s);
        }
    }

    Ok(())
}
