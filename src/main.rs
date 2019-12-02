use std::io;

fn main() -> io::Result<()> {
    println!("{}", advent::day02::part1()?);
    let (x1, x2) = advent::day02::part2()?;
    println!("{} {}", x1, x2);
    Ok(())
}
