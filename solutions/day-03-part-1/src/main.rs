use std::collections::hash_set::HashSet;
use std::fs;

fn char_to_priority(char: char) -> u32 {
    let code = char as u32;

    if char.is_ascii_uppercase() {
        code - 38
    } else if char.is_ascii_lowercase() {
        code - 96
    } else {
        panic!();
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/day-03.txt").unwrap();

    let priorites: u32 = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            let first_set: HashSet<char> = HashSet::from_iter(first.chars());
            let second_set: HashSet<char> = HashSet::from_iter(second.chars());

            let intersection_letter = first_set.intersection(&second_set).next().unwrap();

            char_to_priority(*intersection_letter)
        })
        .sum();

    println!("{}", priorites);
}
