use crate::intcode::ProgramState;
use crate::my_error::MyResult;

pub fn part1() -> MyResult<()> {
    let ps = ProgramState::init_from_file("data/input23.txt")?;

    let mut computers: Vec<_> = (0..50).map(|_| ps.clone()).collect();
    let mut mailboxes: Vec<Vec<(isize, isize)>> = (0..50).map(|_| Vec::new()).collect();

    for (i, computer) in computers.iter_mut().enumerate() {
        let inp: &[isize] = &[i as isize];
        let (out, _) = computer.run_with_input(inp)?;
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
            let (out, _) = computers[i].run_with_input(&inp)?;
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

pub fn part2() -> MyResult<()> {
    let ps = ProgramState::init_from_file("data/input23.txt")?;

    let mut computers: Vec<_> = (0..50).map(|_| ps.clone()).collect();
    let mut mailboxes: Vec<Vec<(isize, isize)>> = (0..50).map(|_| Vec::new()).collect();

    let mut nat: Option<(isize, isize)> = None;

    for (i, computer) in computers.iter_mut().enumerate() {
        let inp: &[isize] = &[i as isize];
        let (out, _) = computer.run_with_input(inp)?;
        assert!(out.len() % 3 == 0);
        for j in 0..out.len() / 3 {
            let (addr, x, y) = (out[3 * j], out[3 * j + 1], out[3 * j + 2]);
            mailboxes[addr as usize].push((x, y));
        }
    }
    let mut last_nat_msg: Option<isize> = None;
    loop {
        let mut idle = true;
        for i in 0..50 {
            let inp: Vec<isize> = if mailboxes[i].is_empty() {
                vec![-1]
            } else {
                idle = false;
                let (x, y) = mailboxes[i].remove(0);
                vec![x, y]
            };
            let (out, _) = computers[i].run_with_input(&inp)?;
            assert!(out.len() % 3 == 0);
            for j in 0..out.len() / 3 {
                idle = false;
                let (addr, x, y) = (out[3 * j], out[3 * j + 1], out[3 * j + 2]);
                if addr == 255 {
                    nat = Some((x, y));
                } else {
                    mailboxes[addr as usize].push((x, y));
                }
            }
        }
        if idle {
            println!("nat says {:?}", nat);
            let nat_msg = nat.unwrap();
            if last_nat_msg == Some(nat_msg.1) {
                break;
            }
            last_nat_msg = Some(nat_msg.1);
            mailboxes[0].push(nat_msg);
        }
    }

    Ok(())
}
