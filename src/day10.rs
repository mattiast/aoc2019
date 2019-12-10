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

use num::integer::gcd;
fn visible(p1: Point, p2: Point, aset: &HashSet<Point>) -> bool {
    if p1 == p2 {
        return false;
    }
    let (a, b) = p1;
    let (x, y) = p2;
    let d = gcd(x - a, y - b);
    let (ux, uy) = ((x - a) / d, (y - b) / d);
    for i in 1..d {
        if aset.contains(&(a + i * ux, b + i * uy)) {
            return false;
        }
    }
    true
}

use std::f64;
fn angle(p: Point, q: Point) -> f64 {
    let (x, y) = (p.0 - q.0, p.1 - q.1);
    let a = Complex::new(y as f64, -x as f64).arg();
    if a < 0.0 {
        a + 2.0 * f64::consts::PI
    } else {
        a
    }
}

use num::complex::Complex;
use std::collections::HashSet;
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
