use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq)]
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

use num::integer::lcm;
pub fn part2() {
    let mut moons_x = vec![
        Position { pos: -8, vel: 0 },
        Position { pos: -5, vel: 0 },
        Position { pos: 11, vel: 0 },
        Position { pos: 1, vel: 0 },
    ];
    let mut moons_y = vec![
        Position { pos: -9, vel: 0 },
        Position { pos: 2, vel: 0 },
        Position { pos: 8, vel: 0 },
        Position { pos: -4, vel: 0 },
    ];
    let mut moons_z = vec![
        Position { pos: -7, vel: 0 },
        Position { pos: -1, vel: 0 },
        Position { pos: -14, vel: 0 },
        Position { pos: -11, vel: 0 },
    ];
    let x_round = get_round_length(&mut moons_x);
    let y_round = get_round_length(&mut moons_y);
    let z_round = get_round_length(&mut moons_z);
    println!("{}", lcm(x_round, lcm(y_round, z_round)));
}

//  <x=-8, y=-9, z=-7>
//  <x=-5, y=2, z=-1>
//  <x=11, y=8, z=-14>
//  <x=1, y=-4, z=-11>
