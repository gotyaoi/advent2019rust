use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let (wire1, wire2): (Vec<(char, u64)>, Vec<(char, u64)>) = {
        let reader = BufReader::new(File::open("../3.txt").unwrap());
        let mut lines = reader.lines();
        (lines.next().unwrap().unwrap().split(',').map(|y| {let mut chars = y.chars(); (chars.next().unwrap(), chars.as_str().parse().unwrap())}).collect(),
         lines.next().unwrap().unwrap().split(',').map(|y| {let mut chars = y.chars(); (chars.next().unwrap(), chars.as_str().parse().unwrap())}).collect())
    };

    let mut wire1_map: HashMap<(i64, i64), u64> = HashMap::new();
    let mut wire2_map: HashMap<(i64, i64), u64> = HashMap::new();
    let mut wire1_set: HashSet<(i64, i64)> = HashSet::new();
    let mut wire2_set: HashSet<(i64, i64)> = HashSet::new();

    let mut x = 0;
    let mut y = 0;
    let mut distance = 0;
    for &(direction, length) in wire1.iter() {
        match direction {
            'U' => {
                for _ in 0..length {
                    distance += 1;
                    y += 1;
                    wire1_map.insert((x, y), distance);
                    wire1_set.insert((x, y));
                }
            },
            'R' => {
                for _ in 0..length {
                    distance += 1;
                    x += 1;
                    wire1_map.insert((x, y), distance);
                    wire1_set.insert((x, y));
                }
            },
            'D' => {
                for _ in 0..length {
                    distance += 1;
                    y -= 1;
                    wire1_map.insert((x, y), distance);
                    wire1_set.insert((x, y));
                }
            },
            'L' => {
                for _ in 0..length {
                    distance += 1;
                    x -= 1;
                    wire1_map.insert((x, y), distance);
                    wire1_set.insert((x, y));
                }
            },
            x => panic!("Bad direction: {}", x),
        }
    }
    x = 0;
    y = 0;
    distance = 0;
    for &(direction, length) in wire2.iter() {
        match direction {
            'U' => {
                for _ in 0..length {
                    distance += 1;
                    y += 1;
                    wire2_map.insert((x, y), distance);
                    wire2_set.insert((x, y));
                }
            },
            'R' => {
                for _ in 0..length {
                    distance += 1;
                    x += 1;
                    wire2_map.insert((x, y), distance);
                    wire2_set.insert((x, y));
                }
            },
            'D' => {
                for _ in 0..length {
                    distance += 1;
                    y -= 1;
                    wire2_map.insert((x, y), distance);
                    wire2_set.insert((x, y));
                }
            },
            'L' => {
                for _ in 0..length {
                    distance += 1;
                    x -= 1;
                    wire2_map.insert((x, y), distance);
                    wire2_set.insert((x, y));
                }
            },
            x => panic!("Bad direction: {}", x),
        }
    }
    let mut crossings = wire1_set.intersection(&wire2_set);
    let mut best = if let Some((x, y)) = crossings.next() {
        (x.abs() + y.abs()) as u64
    }
    else {
        panic!("No crossings.")
    };
    for (x, y) in crossings {
        distance = (x.abs() + y.abs()) as u64;
        if distance < best {
            best = distance;
        }
    }
    println!("{}", best);

    let mut crossings = wire1_set.intersection(&wire2_set);
    let mut best = if let Some(crossing) = crossings.next() {
        wire1_map[crossing] + wire2_map[crossing]
    }
    else {
        panic!("No crossings.")
    };
    for crossing in crossings {
        distance = wire1_map[crossing] + wire2_map[crossing];
        if distance < best {
            best = distance;
        }
    }
    println!("{}", best);
}
