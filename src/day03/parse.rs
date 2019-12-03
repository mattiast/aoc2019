use super::types::{Direction, Wires};
use std::fs::File;
use std::io;
use std::io::prelude::BufRead;
use std::io::BufReader;

fn parse_wires(buf: &str) -> Wires {
    let mut wires: Wires = vec![];
    for wire in buf.split(',') {
        let chars: Vec<char> = wire.chars().collect();
        let dir = match chars[0] {
            'D' => Direction::D,
            'U' => Direction::U,
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("wrong"),
        };
        let s: String = chars[1..].iter().collect();
        let i: i32 = s.trim_end().parse::<i32>().unwrap();

        wires.push((dir, i));
    }
    wires
}

pub fn read_input() -> io::Result<(Wires, Wires)> {
    let file = File::open("data/input03.txt")?;
    let mut reader = BufReader::new(file);

    let mut buf = String::with_capacity(200);
    reader.read_line(&mut buf)?;
    let w1 = parse_wires(&buf);

    buf.clear();
    reader.read_line(&mut buf)?;
    let w2 = parse_wires(&buf);

    Ok((w1, w2))
}