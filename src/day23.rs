use crate::intcode::ProgramState;
use std::io;

pub fn part1() -> io::Result<()> {
    let ps = ProgramState::init_from_file("data/input23.txt")?;

    let mut computers: Vec<_> = (0..50).map(|_| ps.clone()).collect();
    let mut mailboxes: Vec<Vec<(isize, isize)>> = (0..50).map(|_| Vec::new()).collect();

    for (i, computer) in computers.iter_mut().enumerate() {
        let inp: &[isize] = &[i as isize];
        let (out, _) = computer.run_with_input(inp).unwrap();
        assert!(out.len() % 3 == 0);
        for j in 0..out.len() / 3 {
            let (addr, x, y) = (out[3 * j], out[3 * j + 1], out[3 * j + 2]);
            mailboxes[addr as usize].push((x, y));
        }
    }
    'outer: loop {
        for i in 0..50 {
            let inp: Vec<isize> = if mailboxes[i].is_empty() {
                vec![-1]
            } else {
                let (x, y) = mailboxes[i].remove(0);
                vec![x, y]
            };
            let (out, _) = computers[i].run_with_input(&inp).unwrap();
            assert!(out.len() % 3 == 0);
            for j in 0..out.len() / 3 {
                let (addr, x, y) = (out[3 * j], out[3 * j + 1], out[3 * j + 2]);
                if addr == 255 {
                    println!("Sent {:?} to 255", (x, y));
                    break 'outer;
                }
                mailboxes[addr as usize].push((x, y));
            }
        }
    }

    Ok(())
}
