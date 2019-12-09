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

fn decode(i: &mut Vec<i64>, pc: usize, offset: i64) -> (i64, Vec<i64>, usize) {
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
        9 => 1,
        99 => 0,
        x => panic!("Bad opcode: {}", x),
    };
    let mut parameters = Vec::new();
    let mut pos;
    let mut indirect;
    let mut mode;
    for j in 1..=num {
        pos = 10 * 10i64.pow(j as u32);
        mode = (code / pos) % 10;
        if mode == 1 {
            parameters.push(i[pc+j]);
        }
        else if mode == 2 {
            indirect = i[pc+j] as usize;
            if offset < 0 {
                indirect = indirect - (offset.abs() as usize);
            }
            else {
                indirect = indirect + (offset as usize);
            }
            if indirect >= i.len() {
                parameters.push(0);
            }
            else {
                parameters.push(i[indirect]);
            }
        }
        else {
            indirect = i[pc+j] as usize;
            if indirect >= i.len() {
                parameters.push(0);
            }
            else {
                parameters.push(i[indirect]);
            }
        }
    }
    let mut dest = 0;
    if opcode == 1 || opcode == 2 || opcode == 3 || opcode == 7 || opcode == 8 {
        pos = 10 * 10i64.pow((num+1) as u32);
        mode = (code / pos) % 10;
        if mode == 1 {
            panic!("Immediate destination.");
        }
        else if mode == 2 {
            if offset < 0 {
                dest = i[pc+num+1] as usize - offset.abs() as usize;
            }
            else {
                dest = i[pc+num+1] as usize + offset as usize;
            }
        }
        else {
            dest = i[pc+num+1] as usize;
        }
        if dest >= i.len() {
            let needed = dest - i.len() + 1;
            for _ in 0..needed {
                i.push(0);
            }
        }
    }
    (opcode, parameters, dest)
}

pub fn intcode_v2(i: &Vec<i64>) -> i64 {
    let mut i = i.clone();
    let mut pc = 0;
    let mut buf = String::new();
    loop {
        let (opcode, parameters, dest) = decode(&mut i, pc, 0);
        match opcode {
            1 => {
                i[dest] = parameters[0] + parameters[1];
                pc += 4;
            },
            2 => {
                i[dest] = parameters[0] * parameters[1];
                pc += 4;
            },
            3 => {
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
                if parameters[0] < parameters[1] {
                    i[dest] = 1;
                }
                else {
                    i[dest] = 0;
                }
                pc += 4;
            },
            8 => {
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
    offset: i64,
}

impl IntCode_V3 {
    pub fn new(i: &Vec<i64>) -> IntCode_V3 {
        IntCode_V3 {
            memory: i.clone(),
            pc: 0,
            input: None,
            offset: 0,
        }
    }

    pub fn set_input(&mut self, input: i64) {
        self.input = Some(input);
    }

    pub fn process(&mut self) -> State {
        loop {
            let (opcode, parameters, dest) = decode(&mut self.memory, self.pc, self.offset);
            match opcode {
                1 => {
                    self.memory[dest] = parameters[0] + parameters[1];
                    self.pc += 4;
                },
                2 => {
                    self.memory[dest] = parameters[0] * parameters[1];
                    self.pc += 4;
                },
                3 => {
                    if let Some(i) = self.input {
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
                    if parameters[0] < parameters[1] {
                        self.memory[dest] = 1;
                    }
                    else {
                        self.memory[dest] = 0;
                    }
                    self.pc += 4;
                },
                8 => {
                    if parameters[0] == parameters[1] {
                        self.memory[dest] = 1;
                    }
                    else {
                        self.memory[dest] = 0;
                    }
                    self.pc += 4;
                },
                9 => {
                    self.offset += parameters[0];
                    self.pc += 2
                }
                99 => return State::Done,
                x => panic!("Bad opcode: {}.", x),
            }
        }
    }
}
