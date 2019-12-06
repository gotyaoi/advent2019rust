use std::fs;

fn main() {
    let contents = fs::read_to_string("../1.txt")
        .expect("Something went wrong reading the file");

    let mut masses: Vec<u64> = contents.split_whitespace().map(|x| x.parse().unwrap()).collect();

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
