mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod vm;

fn main() {
    let day = 9;

    match day {
        1 => {
            day1::puzzle1();
            day1::puzzle2();
        }
        2 => {
            day2::puzzle1();
            day2::puzzle2();
        }
        3 => {
            day3::puzzle1();
            day3::puzzle2();
        }
        4 => {
            day4::puzzle1();
            day4::puzzle2();
        }
        5 => {
            day5::puzzle1();
        }
        6 => {
            day6::puzzle1();
        }
        7 => {
            day7::puzzle1();
            day7::puzzle2();
        }
        8 => {
            day8::puzzle1();
            day8::puzzle2();
        }
        9 => {
            day9::puzzle1();
            day9::puzzle2();
        }
        _ => unreachable!(),
    }
}
