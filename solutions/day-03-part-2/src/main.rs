use std::collections::hash_set::HashSet;
use std::fs;
use std::hash::Hash;

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

    let mut lines = input.lines();

    let priorities = std::iter::from_fn(move || {
        let next_three = (lines.next(), lines.next(), lines.next());

        if next_three.0.is_some() && next_three.1.is_some() && next_three.2.is_some() {
            Some((
                next_three.0.unwrap(),
                next_three.1.unwrap(),
                next_three.2.unwrap(),
            ))
        } else {
            None
        }
    })
    .map(|lines| {
        // Map to char in common

        let sets: (HashSet<char>, HashSet<char>, HashSet<char>) = (
            HashSet::from_iter(lines.0.chars()),
            HashSet::from_iter(lines.1.chars()),
            HashSet::from_iter(lines.2.chars()),
        );

        // Find the intersection
        sets.0
            .into_iter()
            .filter(|char| sets.1.contains(char) && sets.2.contains(char))
            .next()
            .unwrap()
    })
    .map(|char| char_to_priority(char))
    .sum::<u32>();

    println!("{}", priorities);
}
