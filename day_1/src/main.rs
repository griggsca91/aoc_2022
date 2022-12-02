use std::num::ParseIntError;

fn main() {
    if std::env::args().len() == 1 {
        day_one();
    }

    for arg in std::env::args() {
        match arg.as_str() {
            "--day_one" => day_one(),
            "--day_two" => day_two(),
            _ => {},
        }
    }
}

fn day_two() {
    let mut thicc_elves = vec![0, 0, 0];
    let contents = std::fs::read_to_string("input_1.txt").unwrap();

    let mut tmp_cal_sum = 0;
    for line in contents.split("\n") {
        match line.parse::<i32>() {
            Ok(cal) => tmp_cal_sum += cal,
            Err(_) => {
                let smallest_max_cal = thicc_elves.pop().unwrap();
                if smallest_max_cal < tmp_cal_sum {
                    thicc_elves.push(tmp_cal_sum);
                } else {
                    thicc_elves.push(smallest_max_cal);
                }
                thicc_elves.sort_by(|a, b| b.cmp(a));
                tmp_cal_sum = 0;
            }
        };
    }

    println!("{:?}", thicc_elves);
    println!("max calories {:?}", thicc_elves.iter().sum::<i32>());
}

fn day_one() {
    let contents = std::fs::read_to_string("input_1.txt").unwrap();

    let mut tmp_calorie_sum = 0;
    let mut max_calories = 0;
    for line in contents.split("\n") {
        match line.parse::<i32>() {
            Ok(cal) => tmp_calorie_sum += cal,
            Err(_) => {
                if max_calories < tmp_calorie_sum {
                    max_calories = tmp_calorie_sum;
                }
                tmp_calorie_sum = 0;
            }
        };
    }
    println!("max calories {}", max_calories);
}