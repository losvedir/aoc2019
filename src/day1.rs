use std::fs;

pub fn puzzle1() {
  let fuel: i32 = fs::read_to_string("src/day1input.txt")
    .unwrap()
    .split_whitespace()
    .map(|nstr| nstr.parse::<i32>().unwrap())
    .map(|n| calc1(n))
    .sum();
  println!("day1 puzzle1: {}", fuel);
}

pub fn puzzle2() {
  let fuel: i32 = fs::read_to_string("src/day1input.txt")
    .unwrap()
    .split_whitespace()
    .map(|nstr| nstr.parse::<i32>().unwrap())
    .map(|n| calc2(n))
    .sum();

  println!("day1 puzzle2: {}", fuel);
}

fn calc1(n: i32) -> i32 {
  n / 3 - 2
}
fn calc2(n: i32) -> i32 {
  let mut fuel: i32 = 0;
  let mut part: i32 = n / 3 - 2;
  while part > 0 {
    fuel += part;
    part = part / 3 - 2;
  }
  fuel
}
#[test]
fn test_calc1() {
  assert_eq!(calc1(12), 2);
  assert_eq!(calc1(14), 2);
  assert_eq!(calc1(1969), 654);
  assert_eq!(calc1(100756), 33583);
}
#[test]
fn test_calc2() {
  assert_eq!(calc2(14), 2);
  assert_eq!(calc2(1969), 966);
  assert_eq!(calc2(100756), 50346);
}
