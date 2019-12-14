use crate::vm::VM;
use std::sync::mpsc::channel;
use std::thread;

pub fn puzzle1() {
    let mut max = 0;

    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }
            for c in 0..5 {
                if c == b || c == a {
                    continue;
                }
                for d in 0..5 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 0..5 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        let out = amp_sequence(a, b, c, d, e);
                        if out > max {
                            max = out;
                        }
                    }
                }
            }
        }
    }

    println!("day7 puzzle1: {}", max);
}

pub fn puzzle2() {
    let mut max = 0;

    for a in 5..10 {
        for b in 5..10 {
            if b == a {
                continue;
            }
            for c in 5..10 {
                if c == b || c == a {
                    continue;
                }
                for d in 5..10 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 5..10 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        let out = amp_sequence(a, b, c, d, e);
                        if out > max {
                            max = out;
                        }
                    }
                }
            }
        }
    }

    println!("day7 puzzle2: {}", max);
}

fn init_mem() -> Vec<i64> {
    "3,8,1001,8,10,8,105,1,0,0,21,34,47,72,81,102,183,264,345,426,99999,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,4,9,9,1002,9,3,9,4,9,99,3,9,102,3,9,9,101,2,9,9,102,5,9,9,1001,9,3,9,1002,9,4,9,4,9,99,3,9,101,5,9,9,4,9,99,3,9,101,3,9,9,1002,9,5,9,101,4,9,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,99"
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect()
}

fn amp_sequence(pa: i64, pb: i64, pc: i64, pd: i64, pe: i64) -> i64 {
    let (a_snd, a_rcv) = channel();
    let (b_snd, b_rcv) = channel();
    let (c_snd, c_rcv) = channel();
    let (d_snd, d_rcv) = channel();
    let (e_snd, e_rcv) = channel();
    let (loop_snd, loop_rcv) = channel();
    let (fn_snd, fn_rcv) = channel();

    a_snd.send(pa).unwrap();
    a_snd.send(0).unwrap();

    b_snd.send(pb).unwrap();
    c_snd.send(pc).unwrap();
    d_snd.send(pd).unwrap();
    e_snd.send(pe).unwrap();

    thread::spawn(move || {
        let mut a = VM::initialize(init_mem(), Some(&a_rcv), Some(&b_snd));
        a.run();
    });

    thread::spawn(move || {
        let mut b: VM = VM::initialize(init_mem(), Some(&b_rcv), Some(&c_snd));
        b.run();
    });

    thread::spawn(move || {
        let mut c: VM = VM::initialize(init_mem(), Some(&c_rcv), Some(&d_snd));
        c.run();
    });

    thread::spawn(move || {
        let mut d: VM = VM::initialize(init_mem(), Some(&d_rcv), Some(&e_snd));
        d.run();
    });

    thread::spawn(move || {
        let mut e: VM = VM::initialize(init_mem(), Some(&e_rcv), Some(&loop_snd));
        e.run();
    });

    let hndl = thread::spawn(move || loop {
        match loop_rcv.recv() {
            Ok(val) => {
                fn_snd.send(val).unwrap();
                let _ = a_snd.send(val);
            }
            Err(_) => break,
        }
    });

    hndl.join().unwrap();
    fn_rcv.try_iter().last().expect("last")
}
