use std::fs;

pub fn puzzle1() {
    let mut fewest_0s = None;
    let mut answer = 0;

    let ns = get_input();

    for chunk in ns.chunks(150) {
        let mut ch_0 = 0;
        let mut ch_1 = 0;
        let mut ch_2 = 0;

        for elem in chunk {
            match elem {
                0 => ch_0 += 1,
                1 => ch_1 += 1,
                2 => ch_2 += 1,
                _ => unreachable!(),
            }
        }

        if fewest_0s == None || fewest_0s.unwrap() > ch_0 {
            fewest_0s = Some(ch_0);
            answer = ch_1 * ch_2;
        };
    }
    println!("day8 puzzle1: {}", answer);
}
pub fn puzzle2() {
    let ns = get_input();
    let mut img: Vec<i32> = vec![];

    for px in 0..150 {
        let mut transparent = true;
        let mut i = px;
        while transparent {
            match ns[i] {
                0 => {
                    img.push(0);
                    transparent = false;
                }
                1 => {
                    img.push(1);
                    transparent = false;
                }
                _ => {}
            }
            i += 150;
        }
    }

    println!("day8 puzzle2: {:?}", img);
}

fn get_input() -> Vec<i32> {
    fs::read_to_string("src/day8input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}
