use std::num::ParseIntError;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    let contents = std::fs::read_to_string("input.txt").unwrap();

    if std::env::args().len() == 1 {
        day_one(&contents);
    }

    for arg in std::env::args() {
        match arg.as_str() {
            "--day_one" => day_one(&contents),
            "--day_two" => day_two(&contents),
            _ => {},
        }
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

fn day_one(input: &String) {
    let mut tmp_calorie_sum = 0;
    let mut max_calories = 0;
    for line in input.split("\n") {
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
    println!("Day 1: max calories {}", max_calories);
}

fn day_two(input: &String) {
    let mut thicc_elves = [0, 0, 0];

    let mut tmp_cal_sum = 0;
    for line in input.split("\n") {
        match line.parse::<i32>() {
            Ok(cal) => tmp_cal_sum += cal,
            Err(_) => {
                if thicc_elves[0] < tmp_cal_sum {
                    thicc_elves[0] = tmp_cal_sum;
                    thicc_elves.sort();
                }
                tmp_cal_sum = 0;
            }
        };
    }
    println!("Day 2: max calories {:?}", thicc_elves.iter().sum::<i32>());
}

