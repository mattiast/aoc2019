use num::complex::Complex;
use num::integer::gcd;
use std::collections::HashSet;
use std::f64;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

type Point = (i32, i32);
pub fn read_input() -> io::Result<Vec<Point>> {
    let file = File::open("data/input10.txt")?;
    let reader = BufReader::new(file);

    let mut asteroids: Vec<(i32, i32)> = vec![];
    for (j, line) in reader.lines().enumerate() {
        let line = line?;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push((i as i32, j as i32));
            }
        }
    }
    Ok(asteroids)
}

fn visible((x1, y1): Point, (x2, y2): Point, aset: &HashSet<Point>) -> bool {
    if (x1, y1) == (x2, y2) {
        return false;
    }
    let d = gcd(x2 - x1, y2 - y1);
    let (ux, uy) = ((x2 - x1) / d, (y2 - y1) / d);
    for i in 1..d {
        if aset.contains(&(x1 + i * ux, y1 + i * uy)) {
            return false;
        }
    }
    true
}

fn angle((x1, y1): Point, (x2, y2): Point) -> f64 {
    let (x, y) = (x1 - x2, y1 - y2);
    let a = Complex::new(y as f64, -x as f64).arg();
    if a < 0.0 {
        a + 2.0 * f64::consts::PI
    } else {
        a
    }
}

pub fn part1() -> io::Result<()> {
    let stuff = read_input()?;
    let aset: HashSet<Point> = stuff.iter().cloned().collect();

    let mut maxc = 0;
    let mut station: Point = (0, 0);
    for p in stuff.iter().cloned() {
        let c = stuff
            .iter()
            .cloned()
            .filter(|q| visible(p, *q, &aset))
            .count();

        if c > maxc {
            maxc = c;
            station = p;
        }
    }
    println!("max count: {} at {:?}", maxc, station);

    let mut angles: Vec<_> = stuff
        .iter()
        .cloned()
        .filter(|q| visible(station, *q, &aset) && *q != station)
        .map(|q| (angle(station, q), q))
        .collect();

    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("200th angle {:?}", angles[199]);

    Ok(())
}
