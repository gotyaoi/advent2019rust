use std::fs;

use rust::intcode_v2;

fn main() {
    let instructions: Vec<i64> = fs::read_to_string("../5.txt").unwrap()
        .trim_end().split(',').map(|x| x.parse().unwrap()).collect();

    intcode_v2(&instructions);
    intcode_v2(&instructions);
}
