use std::io;
fn _main() -> io::Result<()> {
    Ok(())
}

fn main() -> io::Result<()> {
    let x = advent::day02::part1()?;
    println!("Day 2 part 1: {}", x);

    let x = advent::day02::part2()?;
    println!("Day 2 part 2: {:?}", x);

    let r = advent::day05::part1()?;
    println!("Day 5 part 1: {:?}", r);
    let r = advent::day05::part2()?;
    println!("Day 5 part 2: {:?}", r);

    advent::day07::part1()?;
    advent::day07::part2()?;

    let r = advent::day09::part1()?;
    println!("Day 9 part 1: {:?}", r);
    let r = advent::day09::part2()?;
    println!("Day 9 part 2: {:?}", r);

    advent::day11::part1()?;
    Ok(())
}
