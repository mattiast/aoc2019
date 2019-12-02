pub mod day01 {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    pub fn part1() -> io::Result<i32> {
        let file = File::open("data/input01.txt")?;
        let reader = BufReader::new(file);

        let result: i32 = reader
            .lines()
            .map(|line| {
                let i: i32 = line.unwrap().parse().unwrap();
                i / 3 - 2
            })
            .sum();
        Ok(result)
    }

    pub fn part2() -> io::Result<i32> {
        let file = File::open("data/input01.txt")?;
        let reader = BufReader::new(file);

        let mut total: i32 = 0;
        for line in reader.lines() {
            let i = line?.parse::<i32>().unwrap();
            total += calc_total_fuel(i);
        }
        Ok(total)
    }

    fn calc_total_fuel(mut x: i32) -> i32 {
        let mut total = 0;
        while x >= 9 {
            x = x / 3 - 2;
            total += x;
        }
        total
    }

    #[test]
    fn test_calc_total_fuel() {
        assert_eq!(calc_total_fuel(100_756), 50_346);
        assert_eq!(calc_total_fuel(1969), 966);
    }
}

pub mod day02 {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    pub fn read_input() -> io::Result<Vec<usize>> {
        let file = File::open("data/input02.txt")?;
        let mut reader = BufReader::new(file);
        let mut buf = "".to_owned();
        reader.read_line(&mut buf)?;
        let bb = buf.trim_end();
        let numbers: Vec<_> = bb.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        Ok(numbers)
    }

    enum Op {
        Add(usize, usize, usize),
        Mul(usize, usize, usize),
        Terminate,
    }

    fn parse_instruction(program: &Vec<usize>, index: usize) -> Result<Op, &'static str> {
        let opcode = *program.get(index).ok_or("out of bounds")?;
        if opcode == 99 {
            return Ok(Op::Terminate);
        }
        if opcode == 1 {
            let x1 = *program.get(index + 1).ok_or("out of bounds")?;
            let x2 = *program.get(index + 2).ok_or("out of bounds")?;
            let x3 = *program.get(index + 3).ok_or("out of bounds")?;
            return Ok(Op::Add(x1, x2, x3));
        }
        if opcode == 2 {
            let x1 = *program.get(index + 1).ok_or("out of bounds")?;
            let x2 = *program.get(index + 2).ok_or("out of bounds")?;
            let x3 = *program.get(index + 3).ok_or("out of bounds")?;
            return Ok(Op::Mul(x1, x2, x3));
        }
        Err("invalid opcode")
    }

    fn execute_instruction(program: &mut Vec<usize>, index: usize) -> Result<Option<usize>, &str> {
        let inst = parse_instruction(&program, index)?;
        match inst {
            Op::Terminate => Ok(None),
            Op::Add(i1, i2, i3) => {
                let x1 = program.get(i1).ok_or("out of bounds")?;
                let x2 = program.get(i2).ok_or("out of bounds")?;
                program[i3] = x1 + x2;
                Ok(Some(index + 4))
            }
            Op::Mul(i1, i2, i3) => {
                let x1 = program.get(i1).ok_or("out of bounds")?;
                let x2 = program.get(i2).ok_or("out of bounds")?;
                program[i3] = x1 * x2;
                Ok(Some(index + 4))
            }
        }
    }

    pub fn part1() -> io::Result<usize> {
        let mut x = read_input()?;
        x[1] = 12;
        x[2] = 2;
        let mut i: usize = 0;
        loop {
            let new_i = execute_instruction(&mut x, i).unwrap();
            match new_i {
                None => break,
                Some(ii) => {
                    i = ii;
                }
            }
        }
        Ok(x[0])
    }

    #[test]
    fn test_execute_instruction() {
        let mut program: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        execute_instruction(&mut program, 0).unwrap();
        assert_eq!(program[5], 9801);
    }
}
