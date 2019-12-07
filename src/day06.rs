use std::collections::{HashMap, HashSet};

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

pub struct Tree {
    pub children: HashMap<String, HashSet<String>>,
    pub parents: HashMap<String, String>,
}

impl Tree {
    fn get_children(&self, node: String) -> Option<&HashSet<String>> {
        self.children.get(&node)
    }
}

pub fn tree_stuff(input: Vec<(String, String)>) -> Tree {
    let mut children: HashMap<String, HashSet<String>> = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();

    for (x, y) in input {
        parents.insert(y.clone(), x.clone());
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
    Tree { children, parents }
}

fn find_depths(t: &Tree, root: String) -> HashMap<String, usize> {
    let mut depths: HashMap<String, usize> = HashMap::new();
    depths.insert(root.clone(), 0);

    let mut to_process = Vec::new();
    to_process.push(root);

    while !to_process.is_empty() {
        let node = to_process.pop().unwrap();
        let d = *depths.get(&node).unwrap();
        let mcs = t.get_children(node);
        if let Some(cs) = mcs {
            for c in cs {
                depths.insert(c.clone(), d + 1);
                to_process.push(c.clone());
            }
        }
    }

    depths
}

fn find_ancestors<'a>(tree: &'a Tree, mut node: &'a str) -> HashMap<String, usize> {
    let mut depths: HashMap<String, usize> = HashMap::new();
    let mut d = 0;
    while let Some(parent) = tree.parents.get(node) {
        node = parent;
        d += 1;
        depths.insert(node.to_owned(), d);
    }
    depths
}

fn find_distance<'a>(tree: &'a Tree, node1: &'a str, node2: &'a str) -> usize {
    let a1 = find_ancestors(&tree, node1);
    let a2 = find_ancestors(&tree, node2);

    let s1: HashSet<String> = a1.keys().cloned().collect();
    let s2: HashSet<String> = a2.keys().cloned().collect();

    s1.intersection(&s2)
        .map(|node| a1.get(node).unwrap() + a2.get(node).unwrap())
        .min()
        .unwrap()
}

pub fn run_stuff() {
    let input = read_input().unwrap();
    let tree = tree_stuff(input);
    let depths = find_depths(&tree, "COM".to_owned());

    let total: usize = depths.iter().map(|(_, y)| *y).sum();
    println!("total {}", total);

    let dist = find_distance(&tree, "YOU", "SAN");
    println!("distance = {}", dist - 2);
}
