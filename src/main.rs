mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let day = 4;

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
        _ => unreachable!(),
    }
}
