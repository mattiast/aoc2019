use std::io;

fn main() -> io::Result<()> {
    println!("{}", advent::day01::part1()?);
    println!("{}", advent::day01::part2()?);
    Ok(())
}
