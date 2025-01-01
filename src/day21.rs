use crate::intcode::ProgramState;
use crate::my_error::MyResult;

use std::fs::File;
use std::io::Read;
pub fn part1(path: &str) -> MyResult<()> {
    let mut ps = ProgramState::init_from_file("data/input21.txt")?;
    let (prompt, _) = ps.run_with_input(&[])?;

    let s = String::from_utf8(prompt.iter().map(|x| *x as u8).collect()).unwrap();
    print!("Prompt: {}", s);

    let input: Vec<_> = {
        let mut file = File::open(path)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        data.into_iter().map(|x| x as isize).collect()
    };
    let (stuff, n) = ps.run_with_input(&input)?;
    println!("Check that all input was read: {} == {}", n, input.len());
    let s = String::from_utf8(stuff.iter().map(|x| *x as u8).collect()).unwrap();
    print!("{}", s);

    if let Some(x) = stuff.last() {
        println!("output {}", x);
    }
    Ok(())
}
