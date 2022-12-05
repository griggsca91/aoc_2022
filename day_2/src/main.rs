use std::num::ParseIntError;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    let contents = std::fs::read_to_string("input.txt").unwrap();

    if std::env::args().len() == 1 {
        part_one(&contents);
    }

    for arg in std::env::args() {
        match arg.as_str() {
            "--part_one" => part_one(&contents),
            "--part_two" => part_two(&contents),
            _ => {},
        }
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

#[derive(Debug, PartialEq, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}



impl RockPaperScissors {
    fn new(s: &str) -> Option<RockPaperScissors> {
        match s {
             "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None
        }
    }

    fn shoot(&self, opponent: Self) -> u32 {
        if *self == opponent {
            return 3 + self.value();
        }

        let resultScore = match (self, opponent) {
            (Self::Rock, Self::Paper) => 0,
            (Self::Rock, Self::Scissors) => 6,
            (Self::Paper, Self::Rock) => 6,
            (Self::Paper, Self::Scissors) => 0,
            (Self::Scissors, Self::Rock) => 0,
            (Self::Scissors, Self::Paper) => 6,
            (_, _) => todo!(),
        };
        resultScore + self.value()
    }

    fn get_weakness(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => Self::Paper,
            RockPaperScissors::Paper => Self::Scissors,
            RockPaperScissors::Scissors => Self::Rock,
        }
    }

    fn get_strong_against(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => Self::Scissors,
            RockPaperScissors::Paper => Self::Rock,
            RockPaperScissors::Scissors => Self::Paper,
        }
    }

    fn value(&self) -> u32 {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn new(s: &str) -> Option<Outcome> {
        match s {
            "X" => Some(Self::Lose),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None
        }
    }

    fn value(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0
        }
    }
}

fn part_one(input: &String) {
    let mut score = 0;
    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();

        let opponent = RockPaperScissors::new(parts[0]).unwrap();
        let player = RockPaperScissors::new(parts[1]).unwrap();

        score += player.shoot(opponent);
    }

    println!("shoot {:?}", score);
}

fn part_two(input: &String) {

    let mut score = 0;
    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();

        let opponent = RockPaperScissors::new(parts[0]).unwrap();
        let expected_outcome = Outcome::new(parts[1]).unwrap();

        let my_sign = match expected_outcome {
            Outcome::Win => opponent.get_weakness(),
            Outcome::Lose => opponent.get_strong_against(),
            Outcome::Draw => opponent,
        };

        score += my_sign.value() + expected_outcome.value();
    }

    println!("shoot {:?}", score);

}

