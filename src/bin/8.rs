use std::fs;

fn main() {
    let data = fs::read_to_string("../8.txt").unwrap();

    let mut layers = Vec::new();

    let mut rest = data.trim_end();

    let mut pair;

    while rest.len() > 150 {
        pair = rest.split_at(150);
        layers.push(pair.0);
        rest = pair.1;
    }
    if rest.len() < 150 {
        panic!("{}", rest.len());
    }
    layers.push(rest);

    let mut zeroes;
    let mut best = 151;
    let mut best_layer = layers[0];
    for layer in layers.iter() {
        zeroes = layer.chars().filter(|&x| x == '0').count();
        if zeroes < best {
            best = zeroes;
            best_layer = layer;
        }
    }
    let counts = best_layer.chars().fold((0, 0), |mut acc, x| {if x == '1' {acc.0 += 1} else if x == '2' {acc.1 += 1} acc});
    println!("{}", counts.0 * counts.1);

    let mut output = String::with_capacity(150);
    let mut x;
    for i in 0..150 {
        for layer in layers.iter() {
            x = &layer[i..i+1];
            if x == "0" {
                output.push(' ');
                break;
            }
            else if x == "1" {
                output.push('X');
                break;
            }
        }
    }
    for i in 0..6 {
        println!("{}", &output[(0+25*i)..(25+25*i)]);
    }
}
