use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn read_input() -> io::Result<Vec<u32>> {
    let file = File::open("data/input08.txt")?;
    let mut reader = BufReader::new(file);
    let mut buf = "".to_owned();
    reader.read_line(&mut buf)?;
    let bb = buf.trim_end();
    let numbers: Vec<_> = bb.chars().map(|s| s.to_digit(10).unwrap()).collect();
    Ok(numbers)
}

pub fn get_layers(input: &[u32]) -> Vec<Vec<Vec<u32>>> {
    input
        .chunks(150)
        .map(|layer| layer.chunks(25).map(|chunk| chunk.to_vec()).collect())
        .collect()
}

pub fn raster_image(layers: Vec<Vec<Vec<u32>>>) -> Vec<Vec<Option<bool>>> {
    let mut image = vec![vec![None; 25]; 6];

    for layer in layers.iter() {
        for i in 0..6 {
            for j in 0..25 {
                if image[i][j].is_none() {
                    image[i][j] = match layer[i][j] {
                        0 => Some(false),
                        1 => Some(true),
                        2 => None,
                        _ => panic!(),
                    }
                }
            }
        }
    }

    image
}

pub fn show_image(image: Vec<Vec<Option<bool>>>) {
    for row in image.iter() {
        let s = row
            .iter()
            .map(|px| match px {
                None => '?',
                Some(true) => '#',
                Some(false) => ' ',
            })
            .collect::<String>();
        println!("{}", s);
    }
}

pub fn part2() -> io::Result<()> {
    let input = read_input()?;
    let layers = get_layers(&input);
    let image = raster_image(layers);
    show_image(image);

    Ok(())
}

pub fn part1() -> io::Result<()> {
    let input = read_input()?;

    let mut counts: Vec<(usize, usize, usize)> = input
        .chunks(150)
        .map(|layer| {
            let (mut z, mut o, mut t) = (0, 0, 0);

            for px in layer {
                match px {
                    0 => {
                        z += 1;
                    }
                    1 => {
                        o += 1;
                    }
                    2 => {
                        t += 1;
                    }
                    _ => panic!(),
                }
            }
            (z, o, t)
        })
        .collect();

    counts.sort();

    for l in counts.iter().take(10) {
        println!("{:?}", l);
    }

    Ok(())
}
