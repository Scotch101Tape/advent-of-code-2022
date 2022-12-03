use std::fs;

enum Sign {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Lose,
    Tie,
}

impl Sign {
    fn against(&self, other: &Self) -> GameResult {
        match self {
            Sign::Rock => match other {
                Sign::Rock => GameResult::Tie,
                Sign::Paper => GameResult::Lose,
                Sign::Scissors => GameResult::Win,
            },
            Sign::Paper => match other {
                Sign::Rock => GameResult::Win,
                Sign::Paper => GameResult::Tie,
                Sign::Scissors => GameResult::Lose,
            },
            Sign::Scissors => match other {
                Sign::Rock => GameResult::Lose,
                Sign::Paper => GameResult::Win,
                Sign::Scissors => GameResult::Tie,
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day-02.txt").unwrap();

    let total_score: i32 = input
        .lines()
        .map(|line| {
            let mut characters = line.split_whitespace();

            let opp = match characters.next().unwrap() {
                "A" => Sign::Rock,
                "B" => Sign::Paper,
                "C" => Sign::Scissors,
                _ => panic!(),
            };

            let you = match characters.next().unwrap() {
                "X" => Sign::Rock,
                "Y" => Sign::Paper,
                "Z" => Sign::Scissors,
                _ => panic!(),
            };

            let winning_score = match you.against(&opp) {
                GameResult::Win => 6,
                GameResult::Tie => 3,
                GameResult::Lose => 0,
            };

            let pick_score = match you {
                Sign::Rock => 1,
                Sign::Paper => 2,
                Sign::Scissors => 3,
            };

            winning_score + pick_score
        })
        .sum();

    println!("{}", total_score);
}
