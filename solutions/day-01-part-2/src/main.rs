use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day-01.txt").unwrap();

    let mut calories: Vec<i32> = input
        .split("\n\n")
        .map(|block| {
            block
                .split_whitespace()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect::<Vec<i32>>();

    calories.sort();

    let top_three_calories: i32 = calories[calories.len() - 3..].iter().sum();

    println!("{}", top_three_calories);
}
