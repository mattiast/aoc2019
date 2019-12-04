mod parse;
mod types;

use parse::read_input;
use types::*;

fn crossing(s1: Segment, s2: Segment) -> Option<(f64, f64, Point)> {
    match (s1, s2) {
        (Segment::Horizontal(x1, x2, y), Segment::Vertical(x, y1, y2)) => {
            let kx = (x - x1) / (x2 - x1);
            let ky = (y - y1) / (y2 - y1);
            if kx > 0. && kx < 1. && ky > 0. && ky < 1. {
                let d1 = kx * (x2 - x1).abs();
                let d2 = ky * (y2 - y1).abs();
                Some((d1, d2, (x, y)))
            } else {
                None
            }
        }
        (Segment::Vertical(x, y1, y2), Segment::Horizontal(x1, x2, y)) => {
            let kx = (x - x1) / (x2 - x1);
            let ky = (y - y1) / (y2 - y1);
            if kx > 0. && kx < 1. && ky > 0. && ky < 1. {
                let d2 = kx * (x2 - x1).abs();
                let d1 = ky * (y2 - y1).abs();
                Some((d1, d2, (x, y)))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn wires_to_segments(wires: &Wires) -> Vec<(f64, Segment)> {
    let mut loc_x: f64 = 0.;
    let mut loc_y: f64 = 0.;
    let mut dist: f64 = 0.0;

    let mut segments: Vec<(f64, Segment)> = vec![];
    for (d, l) in wires.iter() {
        let fl = *l as f64;
        match *d {
            Direction::D => {
                let next = Segment::Vertical(loc_x, loc_y, loc_y - fl);
                loc_y -= fl;
                segments.push((dist, next));
            }
            Direction::U => {
                let next = Segment::Vertical(loc_x, loc_y, loc_y + fl);
                loc_y += fl;
                segments.push((dist, next));
            }
            Direction::R => {
                let next = Segment::Horizontal(loc_x, loc_x + fl, loc_y);
                loc_x += fl;
                segments.push((dist, next));
            }
            Direction::L => {
                let next = Segment::Horizontal(loc_x, loc_x - fl, loc_y);
                loc_x -= fl;
                segments.push((dist, next));
            }
        }
        dist += fl;
    }

    segments
}

fn all_crossings(w1: Wires, w2: Wires) -> Vec<(f64, Point)> {
    let mut crossings: Vec<(f64, Point)> = vec![];

    let ss1 = wires_to_segments(&w1);
    let ss2 = wires_to_segments(&w2);

    for (d1, s1) in ss1.iter() {
        for (d2, s2) in ss2.iter() {
            if let Some((dd1, dd2, p)) = crossing(*s1, *s2) {
                crossings.push((d1 + dd1 + d2 + dd2, p));
            }
        }
    }

    crossings
}

pub fn print_answers() {
    let (wires1, wires2) = read_input().unwrap();
    let cs = all_crossings(wires1, wires2);
    let part1 = cs
        .iter()
        .map(|(_, (x, y))| x.abs() + y.abs())
        .fold(1_000_000f64, |x, y| x.min(y));
    println!("part1 = {}", part1);
    let part2 = cs
        .iter()
        .map(|(d, _)| d)
        .fold(1_000_000f64, |x, y| x.min(*y));
    println!("part2 = {}", part2);
}

#[test]
fn test_crossings() {
    let wires1: Wires = vec![
        (Direction::R, 75),
        (Direction::D, 30),
        (Direction::R, 83),
        (Direction::U, 83),
        (Direction::L, 12),
        (Direction::D, 49),
        (Direction::R, 71),
        (Direction::U, 7),
        (Direction::L, 72),
    ];
    let wires2: Wires = vec![
        (Direction::U, 62),
        (Direction::R, 66),
        (Direction::U, 55),
        (Direction::R, 34),
        (Direction::D, 71),
        (Direction::R, 55),
        (Direction::D, 58),
        (Direction::R, 83),
    ];
    let cs = all_crossings(wires1, wires2);

    let x = cs
        .into_iter()
        .map(|(_, (x, y))| x.abs() + y.abs())
        .fold(1_000_000f64, |x, y| x.min(y));

    assert_eq!(x, 159.);
}
