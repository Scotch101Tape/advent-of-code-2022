use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day-01.txt").unwrap();

    let top_calorie: i32 = input
        .split("\n\n")
        .map(|block| {
            block
                .split_whitespace()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", top_calorie);
}
