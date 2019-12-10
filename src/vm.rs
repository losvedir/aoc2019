use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct VM<'a> {
    pub memory: Vec<i32>,
    iptr: usize,
    halted: bool,
    input: Option<&'a Receiver<i32>>,
    output: Option<&'a Sender<i32>>,
}

impl<'a> VM<'a> {
    pub fn initialize(
        mem: Vec<i32>,
        input: Option<&'a Receiver<i32>>,
        output: Option<&'a Sender<i32>>,
    ) -> VM<'a> {
        VM {
            memory: mem,
            iptr: 0,
            halted: false,
            input: input,
            output: output,
        }
    }

    pub fn step(&mut self) {
        let op = self.memory[self.iptr];
        match op % 100 {
            1 => {
                let im1 = (op / 100) % 10 == 1;
                let im2 = (op / 1000) % 10 == 1;

                let p1 = self.read_relative(1, im1);
                let p2 = self.read_relative(2, im2);

                let p3 = self.read_relative(3, true) as usize;
                self.write(p3, p1 + p2);

                self.iptr += 4;
            }
            2 => {
                let im1 = (op / 100) % 10 == 1;
                let im2 = (op / 1000) % 10 == 1;

                let p1 = self.read_relative(1, im1);
                let p2 = self.read_relative(2, im2);

                let p3 = self.read_relative(3, true) as usize;
                self.write(p3, p1 * p2);

                self.iptr += 4;
            }
            3 => {
                let val: i32 = self.input.unwrap().recv().unwrap();
                let p1 = self.read_relative(1, true) as usize;
                self.write(p1, val);

                self.iptr += 2;
            }
            4 => {
                let val = self.read_relative(1, false);
                self.output.unwrap().send(val).unwrap();
                self.iptr += 2;
            }
            5 => {
                let im1 = (op / 100) % 10 == 1;
                let im2 = (op / 1000) % 10 == 1;

                let p1 = self.read_relative(1, im1);
                let p2 = self.read_relative(2, im2) as usize;
                let do_jmp = p1 != 0;

                if do_jmp {
                    self.iptr = p2;
                } else {
                    self.iptr += 3;
                }
            }
            6 => {
                let im1 = (op / 100) % 10 == 1;
                let im2 = (op / 1000) % 10 == 1;

                let p1 = self.read_relative(1, im1);
                let p2 = self.read_relative(2, im2) as usize;
                let do_jmp = p1 == 0;

                if do_jmp {
                    self.iptr = p2;
                } else {
                    self.iptr += 3;
                }
            }
            7 => {
                let im1 = (op / 100) % 10 == 1;
                let im2 = (op / 1000) % 10 == 1;

                let val1 = self.read_relative(1, im1);
                let val2 = self.read_relative(2, im2);

                let output = if val1 < val2 { 1 } else { 0 };
                let write_loc = self.read_relative(3, true) as usize;
                self.write(write_loc, output);

                self.iptr += 4;
            }
            8 => {
                let im1 = (op / 100) % 10 == 1;
                let im2 = (op / 1000) % 10 == 1;

                let val1 = self.read_relative(1, im1);
                let val2 = self.read_relative(2, im2);

                let output = if val1 == val2 { 1 } else { 0 };
                let write_loc = self.read_relative(3, true) as usize;
                self.write(write_loc, output);

                self.iptr += 4;
            }
            99 => {
                self.halted = true;
            }
            _ => unreachable!(),
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

    fn read_relative(&mut self, n: usize, immediate: bool) -> i32 {
        self.read(self.iptr + n, immediate)
    }

    fn read(&mut self, n: usize, immediate: bool) -> i32 {
        if immediate {
            self.memory[n]
        } else {
            let loc = self.memory[n] as usize;
            self.memory[loc]
        }
    }

    fn write(&mut self, addr: usize, val: i32) {
        self.memory[addr] = val;
    }
}

#[test]
fn test_step() {
    let mut vm = VM::initialize(vec![1, 0, 0, 0, 99], None, None);
    vm.run()
}

#[test]
fn test_immediate() {
    let instr = 1002;
    let (op, im1, im2) = (instr % 10, instr / 100 % 10 == 1, instr / 1000 % 10 == 1);
    assert_eq!(op, 2);
    assert_eq!(im1, false);
    assert_eq!(im2, true);
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
