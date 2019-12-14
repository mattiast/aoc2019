use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::map_res,
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::ops::Index;

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
        input.parse()
    })(input)
}

fn chemical_name(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_uppercase())(input)
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

    Ok((input, Reaction { left: cs, right: c }))
}

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

#[derive(Debug, Clone)]
struct Reaction {
    left: Vec<(usize, String)>,
    right: (usize, String),
}

pub fn part1() {
    let c = read_input();
    println!("num of reactions: {}", c.len());

    let hm: HashMap<String, Reaction> = c.iter().map(|r| (r.right.1.clone(), r.clone())).collect();

    let chemsr: HashSet<_> = c.iter().map(|r| r.right.1.clone()).collect();
    let chemsl: HashSet<_> = c
        .iter()
        .flat_map(|r| r.left.iter().map(|p| p.1.clone()))
        .collect();
    println!("num of different RHS: {}", chemsr.len());
    for s in chemsr.difference(&chemsl) {
        println!("R not L: {}", s);
    }
    for s in chemsl.difference(&chemsr) {
        println!("L not R: {}", s);
    }

    println!("ore need for 1 fuel: {}", ore_need(1, &hm));

    // binary search for trillion
    let mut a: usize = 1;
    let mut b: usize = 10_000_000;
    while b - a > 1 {
        let c = (a + b) / 2;
        if ore_need(c, &hm) < -1_000_000_000_000 {
            b = c;
        } else {
            a = c;
        }
    }
    println!("trillion ore gives fuel: {}", a);
}

fn ore_need(fuel_need: usize, hm: &HashMap<String, Reaction>) -> isize {
    let mut inv = Inventory::init(fuel_need);

    while let Some((c, n)) = inv.next_need() {
        let r = hm.index(&c);
        let times = (n + r.right.0 - 1) / r.right.0;
        inv.apply_reaction(r, times);
    }
    inv.ore
}

struct Inventory {
    have: HashMap<String, isize>,
    ore: isize,
}

impl Inventory {
    fn init(fuel_need: usize) -> Inventory {
        Inventory {
            have: vec![("FUEL".to_owned(), -(fuel_need as isize))]
                .into_iter()
                .collect(),
            ore: 0,
        }
    }

    fn apply_reaction(&mut self, r: &Reaction, times: usize) {
        let er = self.have.entry(r.right.1.clone()).or_insert(0);
        *er += (r.right.0 * times) as isize;

        for (n, c) in r.left.iter() {
            if c == "ORE" {
                self.ore -= (*n * times) as isize;
            } else {
                let p = self.have.entry(c.clone()).or_insert(0);
                *p -= (*n * times) as isize;
            }
        }
    }

    fn next_need(&self) -> Option<(String, usize)> {
        for (c, n) in self.have.iter() {
            if *n < 0 {
                return Some((c.clone(), (-*n) as usize));
            }
        }
        None
    }
}
