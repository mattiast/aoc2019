use std::io;

fn main() -> io::Result<()> {
    println!("{}", advent::day02::part1()?);
    Ok(())
}
