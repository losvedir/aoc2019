use crate::vm::VM;
use std::fs;
use std::sync::mpsc::channel;

pub fn puzzle1() {
    let (ch1_snd, ch1_rcv) = channel();
    let (ch2_snd, ch2_rcv) = channel();

    let mut vm = VM::initialize(get_input(), Some(&ch1_rcv), Some(&ch2_snd));

    ch1_snd.send(1).unwrap();
    vm.run();

    println!(
        "day9 puzzle1: {:?}",
        ch2_rcv.try_iter().collect::<Vec<i64>>()
    );
}
pub fn puzzle2() {
    let (ch1_snd, ch1_rcv) = channel();
    let (ch2_snd, ch2_rcv) = channel();

    let mut vm = VM::initialize(get_input(), Some(&ch1_rcv), Some(&ch2_snd));

    ch1_snd.send(2).unwrap();
    vm.run();

    println!(
        "day9 puzzle2: {:?}",
        ch2_rcv.try_iter().collect::<Vec<i64>>()
    );
}

fn get_input() -> Vec<i64> {
    fs::read_to_string("src/day9input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect()
}

#[test]
fn test_examples() {
    let data = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let (_ch1s, ch1r) = channel();
    let (ch2s, ch2r) = channel();
    let mut vm = VM::initialize(data.clone(), Some(&ch1r), Some(&ch2s));
    vm.run();
    assert_eq!(vm.get_memory(), ch2r.try_iter().collect::<Vec<i64>>());

    let data = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let (_ch1s, ch1r) = channel();
    let (ch2s, ch2r) = channel();
    let mut vm = VM::initialize(data.clone(), Some(&ch1r), Some(&ch2s));
    vm.run();
    assert_eq!(
        vec![1219070632396864],
        ch2r.try_iter().collect::<Vec<i64>>()
    );

    let data = vec![104, 1125899906842624, 99];
    let (_ch1s, ch1r) = channel();
    let (ch2s, ch2r) = channel();
    let mut vm = VM::initialize(data.clone(), Some(&ch1r), Some(&ch2s));
    vm.run();
    assert_eq!(
        vec![1125899906842624],
        ch2r.try_iter().collect::<Vec<i64>>()
    );

    let data = vec![1, 1, 1, 10, 4, 10, 99];
    let (_ch1s, ch1r) = channel();
    let (ch2s, ch2r) = channel();
    let mut vm = VM::initialize(data.clone(), Some(&ch1r), Some(&ch2s));
    vm.run();
    assert_eq!(vec![2], ch2r.try_iter().collect::<Vec<i64>>());

    let data = vec![109, 1000, 203, -995, 99];
    let (ch1s, ch1r) = channel();
    let (ch2s, _ch2r) = channel();
    let mut vm = VM::initialize(data.clone(), Some(&ch1r), Some(&ch2s));
    ch1s.send(111).unwrap();
    vm.run();
    assert_eq!(vec![109, 1000, 203, -995, 99, 111], vm.get_memory());

    let data = vec![109, 5, 21101, 100, 10, 2, 99];
    let mut vm = VM::initialize(data, None, None);
    vm.run();
    assert_eq!(vec![109, 5, 21101, 100, 10, 2, 99, 110], vm.get_memory());
}
