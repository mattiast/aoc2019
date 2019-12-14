use itertools::izip;
use nom::{
    bytes::complete::{tag, take_while1},
    combinator::map_res,
    IResult,
};
use num::integer::lcm;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(
        take_while1(|c: char| c.is_ascii_digit() || c == '-'),
        |input: &str| input.parse(),
    )(input)
}

fn parse_vector(input: &str) -> IResult<&str, (i64, i64, i64)> {
    let (input, _) = tag("<x=")(input)?;
    let (input, x) = parse_number(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = parse_number(input)?;
    let (input, _) = tag(", z=")(input)?;
    let (input, z) = parse_number(input)?;
    let (input, _) = tag(">")(input)?;

    Ok((input, (x, y, z)))
}

fn read_input() -> Vec<(i64, i64, i64)> {
    let file = File::open("data/input12.txt").unwrap();
    let reader = BufReader::new(file);

    let mut reactions = vec![];
    for line in reader.lines() {
        let s = line.unwrap();
        let (_, r) = parse_vector(&s).unwrap();
        reactions.push(r);
    }
    reactions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    pos: i64,
    vel: i64,
}

impl Position {
    fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    fn apply_gravity(moons: &mut [Position]) {
        let n = moons.len();
        for i in 0..n {
            for j in 0..n {
                moons[i].vel += match moons[i].pos.cmp(&moons[j].pos) {
                    Ordering::Equal => 0,
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                };
            }
        }
    }

    fn energy(m1: &[Position], m2: &[Position], m3: &[Position]) -> i64 {
        let mut total = 0;
        for (p1, p2, p3) in izip!(m1, m2, m3) {
            let kinetic = p1.vel.abs() + p2.vel.abs() + p3.vel.abs();
            let potential = p1.pos.abs() + p2.pos.abs() + p3.pos.abs();
            total += kinetic * potential;
        }
        total
    }
}

fn one_round(moons: &mut [Position]) {
    Position::apply_gravity(moons);
    for moon in moons {
        moon.move_pos();
    }
}

fn get_round_length(moons: &mut [Position]) -> usize {
    let original: Vec<_> = moons.to_vec();
    one_round(moons);
    let mut n = 1;
    loop {
        let mats = moons.iter().zip(original.iter()).all(|(x, y)| x == y);
        if mats {
            return n;
        }
        one_round(moons);
        n += 1;
    }
}

pub fn part1() {
    let xx = read_input();
    let mut moons_x: Vec<_> = xx.iter().map(|p| Position { pos: p.0, vel: 0 }).collect();
    let mut moons_y: Vec<_> = xx.iter().map(|p| Position { pos: p.1, vel: 0 }).collect();
    let mut moons_z: Vec<_> = xx.iter().map(|p| Position { pos: p.2, vel: 0 }).collect();
    for _ in 0..1000 {
        one_round(&mut moons_x);
        one_round(&mut moons_y);
        one_round(&mut moons_z);
    }
    println!("{:?}", Position::energy(&moons_x, &moons_y, &moons_z));
}

pub fn part2() {
    let xx = read_input();
    let mut moons_x: Vec<_> = xx.iter().map(|p| Position { pos: p.0, vel: 0 }).collect();
    let mut moons_y: Vec<_> = xx.iter().map(|p| Position { pos: p.1, vel: 0 }).collect();
    let mut moons_z: Vec<_> = xx.iter().map(|p| Position { pos: p.2, vel: 0 }).collect();
    let x_round = get_round_length(&mut moons_x);
    let y_round = get_round_length(&mut moons_y);
    let z_round = get_round_length(&mut moons_z);
    println!("{}", lcm(x_round, lcm(y_round, z_round)));
}
