mod day1;
mod day2;

fn main() {
    let day = 2;

    match day {
        1 => {
            day1::puzzle1();
            day1::puzzle2();
        }
        2 => {
            day2::puzzle1();
            day2::puzzle2();
        }
        _ => unreachable!(),
    }
}
