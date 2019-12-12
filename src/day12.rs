use derive_more::{Add, AddAssign};
use std::cmp::Ordering;

#[derive(Add, AddAssign, Clone, Copy)]
struct Vec3<T>(T, T, T);

struct MoonPosition {
    pos: Vec3<i64>,
    vel: Vec3<i64>,
}

impl MoonPosition {
    fn move_pos(&mut self) {
        self.pos += self.vel;
    }

    fn energy(&self) -> i64 {
        let Vec3(x,y,z) = self.pos;
        let potential = x.abs() + y.abs() + z.abs();
        let Vec3(vx,vy,vz) = self.vel;
        let kinetic = vx.abs() + vy.abs() + vz.abs();

        potential * kinetic
    }

}

fn gravity(m1: &MoonPosition, m2: &MoonPosition) -> Vec3<i64> {
    let Vec3(x1, y1, z1) = m1.pos;
    let Vec3(x2, y2, z2) = m2.pos;

    let ax = match x1.cmp(&x2) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    };
    let ay = match y1.cmp(&y2) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    };
    let az = match z1.cmp(&z2) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    };
    Vec3(ax, ay, az)
}

fn apply_gravity(moons: &mut [MoonPosition]) {
    let n = moons.len();
    for i in 0..n {
        for j in 0..n {
            moons[i].vel += gravity(&moons[i], &moons[j]);
        }
    }
}

fn one_round(moons: &mut [MoonPosition]) {
    apply_gravity(moons);
    for moon in moons {
        moon.move_pos();
    }
}

pub fn part1() {
    let mut moons = vec![
        MoonPosition {pos: Vec3(-8, -9, -7), vel: Vec3(0,0,0)},
        MoonPosition {pos: Vec3(-5, 2, -1), vel: Vec3(0,0,0)},
        MoonPosition {pos: Vec3(11, 8, -14), vel: Vec3(0,0,0)},
        MoonPosition {pos: Vec3(1, -4, -11), vel: Vec3(0,0,0)},
    ];
    for _ in 0..1000 {
        one_round(&mut moons);
    }
    let total_energy: i64 = moons.iter().map(|m| m.energy()).sum();
    println!("total energy {:?}", total_energy);
}

//  <x=-8, y=-9, z=-7>
//  <x=-5, y=2, z=-1>
//  <x=11, y=8, z=-14>
//  <x=1, y=-4, z=-11>


