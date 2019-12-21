use crate::intcode::ProgramState;
use std::io;

use std::fs::File;
use std::io::Read;
pub fn part1() -> io::Result<()> {
    let mut ps = ProgramState::init_from_file("data/input21.txt")?;

    let (prompt, _) = ps.run_with_input(&[]).unwrap();

    let s = String::from_utf8(prompt.iter().map(|x| *x as u8).collect()).unwrap();
    print!("{}", s);

    let input: Vec<_> = {
        let mut file = File::open("day21code2.txt")?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        data.into_iter().map(|x| x as isize).collect()
    };
    let (stuff, n) = ps.run_with_input(&input).unwrap();
    println!("{} == {}", n, input.len());
    let s = String::from_utf8(stuff.iter().map(|x| *x as u8).collect()).unwrap();
    print!("{}", s);

    if let Some(x) = stuff.last() {
        println!("output {}", x);
    }
    Ok(())
}
