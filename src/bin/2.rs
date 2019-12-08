use std::fs;

use rust::intcode_v1;

fn main() {
    let mut instructions: Vec<i64> = fs::read_to_string("../2.txt").unwrap()
        .trim_end().split(',').map(|x| x.parse().unwrap()).collect();

    instructions[1] = 12;
    instructions[2] = 2;

    println!("{}", intcode_v1(&instructions));

    'outer: for noun in 0..100 {
        instructions[1] = noun;
        for verb in 0..100 {
            instructions[2] = verb;
            if intcode_v1(&instructions) == 19690720 {
                println!("{}", noun*100+verb);
                break 'outer;
            }
        }
    }

}
