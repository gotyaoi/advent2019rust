use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let orbits: HashMap<String, String> = {
        let reader = BufReader::new(File::open("../6.txt").unwrap());
        let lines = reader.lines().map(|x| x.unwrap());
        lines.map(|x| {let mut pair = x.split(')'); (pair.next_back().unwrap().to_string(), pair.next_back().unwrap().to_string())}).collect()
    };

    let mut count = 0;
    for (_, mut parent) in orbits.iter() {
        count += 1;
        while parent != "COM" {
            count += 1;
            parent = orbits.get(parent).unwrap();
        }
    }
    println!("{}", count);

    let mut parent = orbits.get("YOU").unwrap();
    let mut you_chain = vec![parent];
    while parent != "COM" {
        you_chain.push(parent);
        parent = orbits.get(parent).unwrap();
    }
    you_chain.reverse();
    parent = orbits.get("SAN").unwrap();
    let mut san_chain = vec![parent];
    while parent != "COM" {
        san_chain.push(parent);
        parent = orbits.get(parent).unwrap();
    }
    san_chain.reverse();

    let mut y = you_chain.into_iter();
    let mut s = san_chain.into_iter();
    for (a, b) in y.by_ref().zip(s.by_ref()) {
        if a != b {
            break;
        }
    }
    println!("{}", y.count() + s.count());
}
