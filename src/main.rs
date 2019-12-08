use std::io;
fn main() -> io::Result<()> {
    advent::day08::part1()?;
    advent::day08::part2()?;

    Ok(())
}

fn icmain() -> io::Result<()> {
    let r = advent::day05::part1()?;
    println!("output = {:?}", r);
    let r = advent::day05::part2()?;
    println!("output = {:?}", r);

    advent::day07::part1()?;
    advent::day07::part2()?;
    Ok(())
}
