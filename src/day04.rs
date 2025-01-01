#![allow(clippy::unreadable_literal)]
use itertools::Itertools;

fn admissible_password(w: u64) -> bool {
    let s = format!("{}", w);
    let cs: Vec<_> = s.chars().collect();

    let incr = cs.windows(2).all(|pair| pair[0] <= pair[1]);
    let strictly_incr = cs.windows(2).all(|pair| pair[0] < pair[1]);

    incr && !strictly_incr
}

fn admissible_password2(w: u64) -> bool {
    let s = format!("{}", w);
    let cs: Vec<_> = s.chars().collect();

    let incr = cs.windows(2).all(|pair| pair[0] <= pair[1]);
    let strictly_incr = cs.windows(2).all(|pair| pair[0] < pair[1]);

    let has_g2 = cs
        .into_iter()
        .chunk_by(|x| *x)
        .into_iter()
        .any(|(_, group)| group.count() == 2);

    incr && !strictly_incr && has_g2
}

pub fn part1() -> usize {
    (240298..784956).filter(|p| admissible_password(*p)).count()
}

pub fn part2() -> usize {
    (240298..784956)
        .filter(|p| admissible_password2(*p))
        .count()
}

#[test]
fn test_password_check() {
    assert!(admissible_password(111111));
    assert!(!admissible_password(223450));
    assert!(!admissible_password(123789));
}
