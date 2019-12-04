mod day1;
mod day2;
mod day3;

fn main() {
    let day = 3;

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
        _ => unreachable!(),
    }
}
