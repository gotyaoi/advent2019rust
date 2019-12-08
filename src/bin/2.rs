use std::fs;

fn main() {
    let mut instructions: Vec<i64> = fs::read_to_string("../2.txt").unwrap()
        .trim_end().split(',').map(|x| x.parse().unwrap()).collect();

    instructions[1] = 12;
    instructions[2] = 2;

    println!("{}", intcode(&instructions));

    'outer: for noun in 0..100 {
        instructions[1] = noun;
        for verb in 0..100 {
            instructions[2] = verb;
            if intcode(&instructions) == 19690720 {
                println!("{}", noun*100+verb);
                break 'outer;
            }
        }
    }

}

fn intcode(i: &Vec<i64>) -> i64{
    let mut i = i.clone();
    let mut pc = 0;
    let mut p1;
    let mut p2;
    let mut dest;
    loop {
        match i[pc] {
            1 => {
                p1 = i[pc+1] as usize;
                p2 = i[pc+2] as usize;
                dest = i[pc+3] as usize;
                i[dest] = i[p1] + i[p2];
                pc += 4;
            },
            2 => {
                p1 = i[pc+1] as usize;
                p2 = i[pc+2] as usize;
                dest = i[pc+3] as usize;
                i[dest] = i[p1] * i[p2];
                pc += 4;
            },
            99 => break,
            x => panic!("Bad opcode: {}.", x),
        }
    }
    i[0]
}
