use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let mut masses: Vec<u64> = {
        let reader = BufReader::new(File::open("../1.txt").unwrap());
        reader.lines().map(|x| x.unwrap().parse().unwrap()).collect()
    };

    println!("{}", masses.iter().map(|x| x/3-2).sum::<u64>());

    let mut sum = 0;
    let mut fuel;
    while let Some(mass) = masses.pop() {
        fuel = (mass/3).saturating_sub(2);
        if fuel > 0 {
            sum += fuel;
            masses.push(fuel);
        }
    }
    println!("{}", sum);
}
