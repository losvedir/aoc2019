pub fn puzzle1() {
  let mut start_state = get_input();
  start_state[1] = 12;
  start_state[2] = 2;
  run(&mut start_state);
  println!("day2 puzzle1: {}", start_state[0]);
}

pub fn puzzle2() {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut memory = get_input();
      memory[1] = noun;
      memory[2] = verb;
      run(&mut memory);

      if memory[0] == 19690720 {
        println!("day2 puzzle2: noun {}, verb {}", noun, verb);
        return;
      }
    }
  }

  println!("day2 puzzle2: nothing found (??)");
}

fn get_input() -> Vec<usize> {
  let input_raw = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,5,23,2,10,23,27,2,27,13,31,1,10,31,35,1,35,9,39,2,39,13,43,1,43,5,47,1,47,6,51,2,6,51,55,1,5,55,59,2,9,59,63,2,6,63,67,1,13,67,71,1,9,71,75,2,13,75,79,1,79,10,83,2,83,9,87,1,5,87,91,2,91,6,95,2,13,95,99,1,99,5,103,1,103,2,107,1,107,10,0,99,2,0,14,0";
  let start_state: Vec<usize> = input_raw
    .split(",")
    .map(|c| c.parse::<usize>().unwrap())
    .collect();

  return start_state;
}

fn run(mem: &mut Vec<usize>) {
  let mut i: usize = 0;

  loop {
    match mem[i] {
      1 => {
        let val1_loc = mem[i + 1];
        let val2_loc = mem[i + 2];
        let dest = mem[i + 3];
        mem[dest] = mem[val1_loc] + mem[val2_loc];
      }
      2 => {
        let val1_loc = mem[i + 1];
        let val2_loc = mem[i + 2];
        let dest = mem[i + 3];
        mem[dest] = mem[val1_loc] * mem[val2_loc];
      }
      99 => break,
      _ => unreachable!(),
    }

    i += 4;
  }
}

#[test]
fn test_run() {
  let mut mem1 = vec![1, 0, 0, 0, 99];
  run(&mut mem1);
  assert_eq!(mem1, vec![2, 0, 0, 0, 99]);

  let mut mem2 = vec![2, 3, 0, 3, 99];
  run(&mut mem2);
  assert_eq!(mem2, vec![2, 3, 0, 6, 99]);

  let mut mem3 = vec![2, 4, 4, 5, 99, 0];
  run(&mut mem3);
  assert_eq!(mem3, vec![2, 4, 4, 5, 99, 9801]);

  let mut mem4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
  run(&mut mem4);
  assert_eq!(mem4, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}
