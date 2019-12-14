use std::collections::HashMap;
#[cfg(test)]
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[derive(Debug, PartialEq)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

pub struct VM<'a> {
    memory: HashMap<i64, i64>,
    iptr: i64,
    relative_base: i64,
    halted: bool,
    input: Option<&'a Receiver<i64>>,
    output: Option<&'a Sender<i64>>,
}

impl<'a> VM<'a> {
    pub fn initialize(
        mem: Vec<i64>,
        input: Option<&'a Receiver<i64>>,
        output: Option<&'a Sender<i64>>,
    ) -> VM<'a> {
        let mut memory: HashMap<i64, i64> = HashMap::with_capacity(mem.len());

        for (i, v) in mem.iter().enumerate() {
            memory.insert(i as i64, *v);
        }

        VM {
            memory: memory,
            iptr: 0,
            relative_base: 0,
            halted: false,
            input: input,
            output: output,
        }
    }

    pub fn step(&mut self) {
        let op = self.memory.get(&self.iptr).expect("iptr out of bounds");
        match op % 100 {
            1 => {
                let mode1 = get_mode(op, 1);
                let mode2 = get_mode(op, 2);
                let mode3 = get_mode(op, 3);

                let p1 = self.read(self.iptr + 1, mode1);
                let p2 = self.read(self.iptr + 2, mode2);

                self.write(self.iptr + 3, mode3, p1 + p2);

                self.iptr += 4;
            }
            2 => {
                let mode1 = get_mode(op, 1);
                let mode2 = get_mode(op, 2);
                let mode3 = get_mode(op, 3);

                let p1 = self.read(self.iptr + 1, mode1);
                let p2 = self.read(self.iptr + 2, mode2);

                self.write(self.iptr + 3, mode3, p1 * p2);

                self.iptr += 4;
            }
            3 => {
                let val: i64 = self.input.unwrap().recv().unwrap();
                let mode = get_mode(op, 1);

                self.write(self.iptr + 1, mode, val);

                self.iptr += 2;
            }
            4 => {
                let mode = get_mode(op, 1);
                let val = self.read(self.iptr + 1, mode);
                self.output.unwrap().send(val).unwrap();
                self.iptr += 2;
            }
            5 => {
                let mode1 = get_mode(op, 1);
                let mode2 = get_mode(op, 2);

                let p1 = self.read(self.iptr + 1, mode1);
                let p2 = self.read(self.iptr + 2, mode2);
                let do_jmp = p1 != 0;

                if do_jmp {
                    self.iptr = p2;
                } else {
                    self.iptr += 3;
                }
            }
            6 => {
                let mode1 = get_mode(op, 1);
                let mode2 = get_mode(op, 2);

                let p1 = self.read(self.iptr + 1, mode1);
                let p2 = self.read(self.iptr + 2, mode2);
                let do_jmp = p1 == 0;

                if do_jmp {
                    self.iptr = p2;
                } else {
                    self.iptr += 3;
                }
            }
            7 => {
                let mode1 = get_mode(op, 1);
                let mode2 = get_mode(op, 2);
                let mode3 = get_mode(op, 3);

                let val1 = self.read(self.iptr + 1, mode1);
                let val2 = self.read(self.iptr + 2, mode2);

                let output = if val1 < val2 { 1 } else { 0 };

                self.write(self.iptr + 3, mode3, output);

                self.iptr += 4;
            }
            8 => {
                let mode1 = get_mode(op, 1);
                let mode2 = get_mode(op, 2);
                let mode3 = get_mode(op, 3);

                let val1 = self.read(self.iptr + 1, mode1);
                let val2 = self.read(self.iptr + 2, mode2);

                let output = if val1 == val2 { 1 } else { 0 };

                self.write(self.iptr + 3, mode3, output);

                self.iptr += 4;
            }
            9 => {
                let mode = get_mode(op, 1);
                let val = self.read(self.iptr + 1, mode);
                self.relative_base += val;
                self.iptr += 2;
            }
            99 => {
                self.halted = true;
            }
            _ => {
                println!("heeeeere");
                unreachable!();
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.halted {
                break;
            }

            self.step()
        }
    }

    #[cfg(test)]
    pub fn get_memory(&mut self) -> Vec<i64> {
        let mut output: Vec<i64> = vec![];
        let mut i: i64 = 0;
        let mut elem = self.memory.get(&i);

        while elem != None {
            output.push(*elem.unwrap());
            i += 1;
            elem = self.memory.get(&i);
        }

        output
    }

    fn read(&mut self, n: i64, mode: Mode) -> i64 {
        match mode {
            Mode::Immediate => *self.memory.get(&n).unwrap_or(&0),
            Mode::Position => {
                let loc = *self.memory.get(&n).expect("unexpected loc");
                *self.memory.get(&loc).unwrap_or(&0)
            }
            Mode::Relative => {
                let loc = *self.memory.get(&n).expect("unexpected relloc") + self.relative_base;
                *self.memory.get(&loc).unwrap_or(&0)
            }
        }
    }

    fn write(&mut self, parameter_loc: i64, mode: Mode, val: i64) {
        let write_destination = self.read(parameter_loc, Mode::Immediate);

        match mode {
            Mode::Position => self.memory.insert(write_destination, val),
            Mode::Relative => self
                .memory
                .insert(write_destination + self.relative_base, val),
            _ => unreachable!(),
        };
    }
}

fn get_mode(op: &i64, p: u32) -> Mode {
    match (*op / (i64::pow(10, p + 1))) % 10 {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => unreachable!(),
    }
}

#[test]
fn test_step() {
    let mut vm = VM::initialize(vec![1, 0, 0, 0, 99], None, None);
    vm.run()
}

#[test]
fn test_mode() {
    assert_eq!(get_mode(&1101, 1), Mode::Immediate);
    assert_eq!(get_mode(&1101, 2), Mode::Immediate);
    assert_eq!(get_mode(&1101, 3), Mode::Position);
    assert_eq!(get_mode(&204, 1), Mode::Relative);
}

#[test]
fn test_channels() {
    let (snd1, rcv1) = channel();
    let (snd2, rcv2) = channel();
    let mut vm: VM = VM::initialize(vec![3, 5, 4, 5, 99, 0], Some(&rcv1), Some(&snd2));
    snd1.send(77).unwrap();
    vm.run();
    assert_eq!(rcv2.recv().unwrap(), 77);
}
