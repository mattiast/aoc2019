use std::fs::File;

fn read_input() -> (Wires, Wires) {
    unimplemented!();
}

enum Direction {
    R,
    D,
    L,
    U,
}

type Wires = Vec<(Direction, i32)>;

#[derive(Clone, Copy)]
enum Segment {
    Horizontal(i32, i32, i32),
    Vertical(i32, i32, i32),
}

fn crossing(s1: Segment, s2: Segment) -> Option<(i32, i32)> {
    match (s1, s2) {
        (Segment::Horizontal(x1, x2, y), Segment::Vertical(x, y1, y2)) => {
            if x1 < x && x < x2 && y1 < y && y < y2 {
                Some((x, y))
            }
            else {
                None
            }
        }
        (Segment::Vertical(x, y1, y2), Segment::Horizontal(x1, x2, y)) => {
            if x1 < x && x < x2 && y1 < y && y < y2 {
                Some((x, y))
            }
            else {
                None
            }
        }
        _ => None,
    }
}

fn wires_to_segments(wires: &Wires) -> Vec<Segment> {
    let mut loc_x: i32 = 0;
    let mut loc_y: i32 = 0;

    let mut segments: Vec<Segment> = vec![];
    for (d, l) in wires.iter() {
        match *d {
            Direction::D => {
                let next = Segment::Vertical(loc_x, loc_y - *l, loc_y);
                loc_y -= *l;
                segments.push(next);
            }
            Direction::U => {
                let next = Segment::Vertical(loc_x, loc_y, loc_y + *l);
                loc_y += *l;
                segments.push(next);
            }
            Direction::R => {
                let next = Segment::Horizontal(loc_x, loc_x + *l, loc_y);
                loc_x += *l;
                segments.push(next);
            }
            Direction::L => {
                let next = Segment::Horizontal(loc_x - *l, loc_x, loc_y);
                loc_x -= *l;
                segments.push(next);
            }
        }
    }

    segments
}

fn all_crossings(w1: Wires, w2: Wires) -> Vec<(i32, i32)> {
    let mut crossings: Vec<(i32, i32)> = vec![];

    let ss1 = wires_to_segments(&w1);
    let ss2 = wires_to_segments(&w2);

    for s1 in ss1.iter() {
        for s2 in ss2.iter() {
            if let Some(p) = crossing(*s1, *s2) {
                crossings.push(p);
            }
        }
    }

    crossings
}

pub fn part1() -> i32 {
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
    for c in cs {
        println!("{:?}", c);
    }
    5
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

    let x = cs.into_iter().map(|(x, y)| x.abs() + y.abs()).min();

    assert_eq!(x, Some(159));
}