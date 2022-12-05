use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day-04.txt").unwrap();

    let overlap_count = input
        .lines()
        .map(|line| {
            lazy_static! {
                static ref PAIR_REGEX: Regex = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
            }

            let capture = PAIR_REGEX.captures_iter(line).next().unwrap();

            (
                capture[1].parse::<i32>().unwrap(),
                capture[2].parse::<i32>().unwrap(),
                capture[3].parse::<i32>().unwrap(),
                capture[4].parse::<i32>().unwrap(),
            )
        })
        .filter(|bounds| {
            let first_bounds = (bounds.0, bounds.1);
            let second_bounds = (bounds.2, bounds.3);

            (first_bounds.0 <= second_bounds.0 && first_bounds.1 >= second_bounds.1)
                || (first_bounds.0 >= second_bounds.0 && first_bounds.1 <= second_bounds.1)
        })
        .count();

    println!("{}", overlap_count);
}
