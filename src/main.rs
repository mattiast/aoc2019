use std::io;

fn main() -> io::Result<()> {
    let r = advent::day05::part1()?;
    println!("output = {:?}", r);
    let r = advent::day05::part2()?;
    println!("output = {:?}", r);
    Ok(())
}
