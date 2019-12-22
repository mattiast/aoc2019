#![allow(clippy::unreadable_literal)]
use std::io;

#[derive(Debug)]
pub enum Shuffle {
    DealNewStack,
    Cut(i64),
    DealIncrement(i64),
}

pub fn part1() -> io::Result<()> {
    let shuffles = parsing::read_input()?;
    let mut p = 2019;
    for s in shuffles {
        p = map_position(p, s);
    }
    println!("2019 goes to {}", p);
    Ok(())
}

const N1: i64 = 10007;

fn map_position(p: i64, s: Shuffle) -> i64 {
    match s {
        Shuffle::DealNewStack => N1 - 1 - p,
        Shuffle::Cut(k) => (p - k) % N1,
        Shuffle::DealIncrement(k) => (p * k) % N1,
    }
}

pub fn part2() -> io::Result<()> {
    let shuffles = parsing::read_input()?;
    let prod = {
        let mut o = Operation(1, 0);
        for s in shuffles {
            o = map_operation(s) * o;
        }
        o
    };
    let pow = pow_operation(prod, 101741582076661);
    println!("pow = {:?}", pow);
    Ok(())
}

const N2: i64 = 119315717514047;

#[derive(Clone, Copy, Debug)]
struct Operation(i64, i64);

use std::ops::Mul;

impl Mul for Operation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let Operation(a, b) = self;
        let Operation(c, d) = rhs;
        // a * (c*x+d) + b = (a*c)*x + (a*d+b)
        let aa = ((a as i128) * (c as i128)) % (N2 as i128);
        let bb = (((a as i128) * (d as i128)) + (b as i128)) % (N2 as i128);
        Operation(aa as i64, bb as i64)
    }
}

fn map_operation(s: Shuffle) -> Operation {
    match s {
        Shuffle::DealNewStack => Operation(-1, -1),
        Shuffle::Cut(k) => Operation(1, -k),
        Shuffle::DealIncrement(k) => Operation(k, 0),
    }
}

fn pow_operation(mut x: Operation, mut n: usize) -> Operation {
    let mut result = Operation(1, 0);
    // invariant: x^n * result
    while n > 0 {
        let r = n % 2;
        if r == 1 {
            result = result * x;
        }
        x = x * x;
        n /= 2;
    }
    result
}

mod parsing {
    use super::Shuffle;
    use nom::{
        bytes::complete::{tag, take_while1},
        combinator::map_res,
        IResult,
    };
    use std::fs::File;
    use std::io;
    use std::io::{prelude::BufRead, BufReader};
    fn parse_number(input: &str) -> IResult<&str, i64> {
        map_res(
            take_while1(|c: char| c.is_ascii_digit() || c == '-'),
            |input: &str| input.parse(),
        )(input)
    }

    fn parse_new_stack(input: &str) -> IResult<&str, Shuffle> {
        let (input, _) = tag("deal into new stack")(input)?;
        Ok((input, Shuffle::DealNewStack))
    }
    fn parse_increment(input: &str) -> IResult<&str, Shuffle> {
        let (input, _) = tag("deal with increment ")(input)?;
        let (input, n) = parse_number(input)?;
        Ok((input, Shuffle::DealIncrement(n)))
    }
    fn parse_cut(input: &str) -> IResult<&str, Shuffle> {
        let (input, _) = tag("cut ")(input)?;
        let (input, n) = parse_number(input)?;
        Ok((input, Shuffle::Cut(n)))
    }

    named!(parse_shuffle<&str, Shuffle>, alt!(parse_new_stack | parse_increment | parse_cut));

    pub fn read_input() -> io::Result<Vec<Shuffle>> {
        let file = File::open("data/input22.txt")?;
        let reader = BufReader::new(file);

        let mut reactions = vec![];
        for line in reader.lines() {
            let s = line.unwrap();
            let (_, r) = parse_shuffle(&s).unwrap();
            reactions.push(r);
        }
        Ok(reactions)
    }
}
