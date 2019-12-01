use std::io;


fn main() -> io::Result<()> {
    let a = advent::day01::part1()?;
    println!("{}", a);
    Ok(())
}
