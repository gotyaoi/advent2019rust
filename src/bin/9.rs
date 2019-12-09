use std::fs;

use rust::{State, IntCode_V3};

fn main() {
    let instructions: Vec<i64> = fs::read_to_string("../9.txt").unwrap()
        .trim_end().split(',').map(|x| x.parse().unwrap()).collect();

    let mut test = IntCode_V3::new(&instructions);
    test.set_input(1);
    while let State::Output(x) = test.process() {
        println!("{}", x);
    }
    test = IntCode_V3::new(&instructions);
    test.set_input(2);
    while let State::Output(x) = test.process() {
        println!("{}", x);
    }
}
