use advent::my_error::MyResult;

fn main() -> MyResult<()> {
    advent::day19::part2()?;
    Ok(())
}

fn _main() -> MyResult<()> {
    println!("Day 2 part 1: {:?}", advent::day02::part1()?);
    println!("Day 2 part 2: {:?}", advent::day02::part2()?);

    println!("Day 5 part 1: {:?}", advent::day05::part1()?);
    println!("Day 5 part 2: {:?}", advent::day05::part2()?);

    println!("Day 7 part 1: {:?}", advent::day07::part1()?);
    println!("Day 7 part 2: {:?}", advent::day07::part2()?);

    println!("Day 9 part 1: {:?}", advent::day09::part1()?);
    println!("Day 9 part 2: {:?}", advent::day09::part2()?);

    println!("Day 11 part 1: {:?}", advent::day11::part1()?);
    advent::day11::part2()?;

    println!("Day 19 part 1: {:?}", advent::day19::part1()?);

    Ok(())
}
