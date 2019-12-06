use std::collections::{HashMap, HashSet};

type Tree = HashMap<String, HashSet<String>>;

pub fn tree_stuff(input: Vec<(String, String)>) -> Tree {
    let mut children: Tree = HashMap::new();

    for (x, y) in input {
        match children.get_mut(&x) {
            Some(s) => {
                s.insert(y);
            }
            None => {
                let mut s = HashSet::new();
                s.insert(y);
                children.insert(x, s);
            }
        }
    }
    children
}

fn find_depths(t: &Tree, root: String) -> HashMap<String, usize> {
    let mut depths: HashMap<String, usize> = HashMap::new();
    depths.insert(root.clone(), 0);

    let mut to_process = Vec::new();
    to_process.push(root);

    while !to_process.is_empty() {
        let node = to_process.pop().unwrap();
        let d = *depths.get(&node).unwrap();
        let mcs = t.get(&node);
        if let Some(cs) = mcs {
            for c in cs {
                depths.insert(c.clone(), d + 1);
                to_process.push(c.clone());
            }
        }
    }

    depths
}

use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};
pub fn read_input() -> io::Result<Vec<(String, String)>> {
    let file = File::open("data/input06.txt")?;
    let reader = BufReader::new(file);

    let mut result: Vec<(String, String)> = vec![];
    for line in reader.lines() {
        let rline = line?;
        let mut stars = rline.split(')');
        let x = stars.next().unwrap();
        let y = stars.next().unwrap();
        result.push((x.to_owned(), y.to_owned()));
    }
    Ok(result)
}

pub fn run_stuff() {
    let input = read_input().unwrap();
    let tree = tree_stuff(input);
    let depths = find_depths(&tree, "COM".to_owned());

    let total: usize = depths.iter().map(|(_, y)| *y).sum();
    println!("total {}", total);
}
