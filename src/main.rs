use std::io;
fn amain() -> io::Result<()> {
    advent::day11::part1()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let r = advent::day05::part1()?;
    println!("output = {:?}", r);
    let r = advent::day05::part2()?;
    println!("output = {:?}", r);

    advent::day07::part1()?;
    advent::day07::part2()?;

    advent::day09::part1()?;
    advent::day09::part2()?;
    advent::day11::part1()?;
    Ok(())
}
