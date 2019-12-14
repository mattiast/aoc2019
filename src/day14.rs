use nom::{
    bytes::complete::{take_while1, tag},
    combinator::{map_res},
    character::complete::char,
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| { c.is_ascii_digit() }), |input: &str| { input.parse()})(input)
}

fn chemical_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| { c.is_ascii_uppercase() })(input)
}

fn chemical(input: &str) -> IResult<&str, (usize, String)> {
    let (input, n) = parse_number(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, name) = chemical_name(input)?;

    Ok((input, (n, name.to_owned())))
}

named!(chemicals<&str, Vec<(usize, String)>>, separated_list!(tag(", "), chemical));

fn parse_line(input: &str) -> IResult<&str, Reaction> {
    let (input, cs) = chemicals(input)?;
    let (input, _) = tag(" => ")(input)?;
    let (input, c) = chemical(input)?;

    Ok((input, Reaction {
        left: cs,
        right: c,
    }))
}

use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
fn read_input() -> Vec<Reaction> {
    let file = File::open("data/input14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut reactions = vec![];
    for line in reader.lines() {
        let s = line.unwrap();
        let (_, r) = parse_line(&s).unwrap();
        reactions.push(r);
    }
    reactions
}

#[derive(Debug)]
struct Reaction {
    left: Vec<(usize, String)>,
    right: (usize, String),
}

pub fn part1() {
    let c = read_input();
    println!("{}", c.len());
}
