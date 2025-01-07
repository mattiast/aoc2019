use advent::{day24, my_error::MyResult};
// use clap::{arg, command};

fn _main() -> MyResult<()> {
    advent::day19::part2()?;
    Ok(())
}

fn main() -> MyResult<()> {
    tracing_subscriber::fmt::init();

    // let matches = command!().arg(arg!([file] "file")).get_matches();
    // let path = matches.get_one::<String>("file").unwrap();

    day24::part1()?;
    Ok(())
}
