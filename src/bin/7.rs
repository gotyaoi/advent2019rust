use std::fs;

use rust::{State, IntCode_V3};

fn main() {
    let instructions: Vec<i64> = fs::read_to_string("../7.txt").unwrap()
        .trim_end().split(',').map(|x| x.parse().unwrap()).collect();

    let mut best = 0;
    let mut ampA;
    let mut ampB;
    let mut ampC;
    let mut ampD;
    let mut ampE;
    let mut oA;
    let mut oB;
    let mut oC;
    let mut oD;
    let mut oE;
    for pA in 0..=4 {
        for pB in 0..=4 {
            if pB == pA {
                continue;
            }
            for pC in 0..=4 {
                if pC == pA || pC == pB {
                    continue;
                }
                for pD in 0..=4 {
                    if pD == pA || pD == pB || pD == pC {
                        continue;
                    }
                    for pE in 0..=4 {
                        if pE == pA || pE == pB || pE == pC || pE == pD {
                            continue;
                        }
                        ampA = IntCode_V3::new(&instructions);
                        ampA.set_input(pA);
                        ampA.process();
                        ampA.set_input(0);
                        oA = match ampA.process() {
                            State::Output(x) => x,
                            _ => panic!(),
                        };
                        ampB = IntCode_V3::new(&instructions);
                        ampB.set_input(pB);
                        ampB.process();
                        ampB.set_input(oA);
                        oB = match ampB.process() {
                            State::Output(x) => x,
                            _ => panic!(),
                        };
                        ampC = IntCode_V3::new(&instructions);
                        ampC.set_input(pC);
                        ampC.process();
                        ampC.set_input(oB);
                        oC = match ampC.process() {
                            State::Output(x) => x,
                            _ => panic!(),
                        };
                        ampD = IntCode_V3::new(&instructions);
                        ampD.set_input(pD);
                        ampD.process();
                        ampD.set_input(oC);
                        oD = match ampD.process() {
                            State::Output(x) => x,
                            _ => panic!(),
                        };
                        ampE = IntCode_V3::new(&instructions);
                        ampE.set_input(pE);
                        ampE.process();
                        ampE.set_input(oD);
                        oE = match ampE.process() {
                            State::Output(x) => x,
                            _ => panic!(),
                        };
                        if oE > best {
                            best = oE
                        }
                    }
                }
            }
        }
    }
    println!("{}", best);

    for pA in 5..=9 {
        for pB in 5..=9 {
            if pB == pA {
                continue;
            }
            for pC in 5..=9 {
                if pC == pA || pC == pB {
                    continue;
                }
                for pD in 5..=9 {
                    if pD == pA || pD == pB || pD == pC {
                        continue;
                    }
                    for pE in 5..=9 {
                        if pE == pA || pE == pB || pE == pC || pE == pD {
                            continue;
                        }
                        ampA = IntCode_V3::new(&instructions);
                        ampA.set_input(pA);
                        ampA.process();
                        ampB = IntCode_V3::new(&instructions);
                        ampB.set_input(pB);
                        ampB.process();
                        ampC = IntCode_V3::new(&instructions);
                        ampC.set_input(pC);
                        ampC.process();
                        ampD = IntCode_V3::new(&instructions);
                        ampD.set_input(pD);
                        ampD.process();
                        ampE = IntCode_V3::new(&instructions);
                        ampE.set_input(pE);
                        ampE.process();
                        ampA.set_input(0);
                        oE = 0;
                        loop {
                            oA = match ampA.process() {
                                State::Output(x) => x,
                                State::Done => break,
                                State::Input => panic!(),
                            };
                            ampB.set_input(oA);
                            oB = match ampB.process() {
                                State::Output(x) => x,
                                _ => panic!(),
                            };
                            ampC.set_input(oB);
                            oC = match ampC.process() {
                                State::Output(x) => x,
                                _ => panic!(),
                            };
                            ampD.set_input(oC);
                            oD = match ampD.process() {
                                State::Output(x) => x,
                                _ => panic!(),
                            };
                            ampE.set_input(oD);
                            oE = match ampE.process() {
                                State::Output(x) => x,
                                _ => panic!(),
                            };
                            ampA.set_input(oE)
                        }
                        if oE > best {
                            best = oE
                        }
                    }
                }
            }
        }
    }
    println!("{}", best);
}
