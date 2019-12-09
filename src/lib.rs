use std::io;

pub fn intcode_v1(i: &Vec<i64>) -> i64 {
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

fn decode(i: &Vec<i64>, pc: usize) -> (i64, Vec<i64>) {
    let code = i[pc];
    let opcode = code % 100;
    let num: usize = match opcode {
        1 => 2,
        2 => 2,
        3 => 0,
        4 => 1,
        5 => 2,
        6 => 2,
        7 => 2,
        8 => 2,
        99 => 0,
        x => panic!("Bad opcode: {}", x),
    };
    let mut parameters = Vec::new();
    let mut pos;
    let mut indirect;
    for j in 1..=num {
        pos = 10 * 10i64.pow(j as u32);
        if (code / pos) % 10 == 1 {
            parameters.push(i[pc+j]);
        }
        else {
            indirect = i[pc+j] as usize;
            parameters.push(i[indirect]);
        }
    }
    (opcode, parameters)
}

pub fn intcode_v2(i: &Vec<i64>) -> i64 {
    let mut i = i.clone();
    let mut pc = 0;
    let mut dest;
    let mut buf = String::new();
    loop {
        let (opcode, parameters) = decode(&i, pc);
        match opcode {
            1 => {
                dest = i[pc+3] as usize;
                i[dest] = parameters[0] + parameters[1];
                pc += 4;
            },
            2 => {
                dest = i[pc+3] as usize;
                i[dest] = parameters[0] * parameters[1];
                pc += 4;
            },
            3 => {
                dest = i[pc+1] as usize;
                io::stdin().read_line(&mut buf).unwrap();
                i[dest] = buf.trim_end().parse().unwrap();
                buf.clear();
                pc += 2;
            },
            4 => {
                println!("{}", parameters[0]);
                pc += 2;
            },
            5 => {
                if parameters[0] != 0 {
                    pc = parameters[1] as usize;
                }
                else {
                    pc += 3
                }
            },
            6 => {
                if parameters[0] == 0 {
                    pc = parameters[1] as usize;
                }
                else {
                    pc += 3
                }
            },
            7 => {
                dest = i[pc+3] as usize;
                if parameters[0] < parameters[1] {
                    i[dest] = 1;
                }
                else {
                    i[dest] = 0;
                }
                pc += 4;
            },
            8 => {
                dest = i[pc+3] as usize;
                if parameters[0] == parameters[1] {
                    i[dest] = 1;
                }
                else {
                    i[dest] = 0;
                }
                pc += 4;
            },
            99 => break,
            x => panic!("Bad opcode: {}.", x),
        }
    }
    i[0]
}

pub enum State {
    Input,
    Output(i64),
    Done,
}

pub struct IntCode_V3 {
    memory: Vec<i64>,
    pc: usize,
    input: Option<i64>,
}

impl IntCode_V3 {
    pub fn new(i: &Vec<i64>) -> IntCode_V3 {
        IntCode_V3 {
            memory: i.clone(),
            pc: 0,
            input: None,
        }
    }

    pub fn set_input(&mut self, input: i64) {
        self.input = Some(input);
    }

    pub fn process(&mut self) -> State {
        let mut dest;
        loop {
            let (opcode, parameters) = decode(&self.memory, self.pc);
            match opcode {
                1 => {
                    dest = self.memory[self.pc+3] as usize;
                    self.memory[dest] = parameters[0] + parameters[1];
                    self.pc += 4;
                },
                2 => {
                    dest = self.memory[self.pc+3] as usize;
                    self.memory[dest] = parameters[0] * parameters[1];
                    self.pc += 4;
                },
                3 => {
                    if let Some(i) = self.input {
                        dest = self.memory[self.pc+1] as usize;
                        self.memory[dest] = i;
                        self.input = None;
                        self.pc += 2;
                    }
                    else {
                        return State::Input;
                    }
                },
                4 => {
                    self.pc += 2;
                    return State::Output(parameters[0]);
                },
                5 => {
                    if parameters[0] != 0 {
                        self.pc = parameters[1] as usize;
                    }
                    else {
                        self.pc += 3
                    }
                },
                6 => {
                    if parameters[0] == 0 {
                        self.pc = parameters[1] as usize;
                    }
                    else {
                        self.pc += 3
                    }
                },
                7 => {
                    dest = self.memory[self.pc+3] as usize;
                    if parameters[0] < parameters[1] {
                        self.memory[dest] = 1;
                    }
                    else {
                        self.memory[dest] = 0;
                    }
                    self.pc += 4;
                },
                8 => {
                    dest = self.memory[self.pc+3] as usize;
                    if parameters[0] == parameters[1] {
                        self.memory[dest] = 1;
                    }
                    else {
                        self.memory[dest] = 0;
                    }
                    self.pc += 4;
                },
                99 => return State::Done,
                x => panic!("Bad opcode: {}.", x),
            }
        }
    }
}
