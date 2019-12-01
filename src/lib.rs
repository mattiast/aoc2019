
pub mod day01 {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    pub fn part1() -> io::Result<i32> {
        let file = File::open("data/input01.txt")?;
        let reader = BufReader::new(file);

        let mut total: i32 = 0;
        for line in reader.lines() {
            let i = line?.parse::<i32>().unwrap();
            total += i / 3 - 2;
        }
        Ok(total)
    }
}
