use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, fs};

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let mut stacks = Vec::new();

    for line in input.lines() {
        if line == "\n" {
            break;
        }

        for i in (1..).step_by(4) {
            if i >= line.len() {
                break;
            } else if let Some(char) = line.get(i..i + 1) {
                let char = char.chars().next().unwrap();

                if char.is_numeric() {
                    return stacks;
                } else if char.is_alphabetic() {
                    let crate_i = (i - 1) / 4;

                    while stacks.len() <= crate_i {
                        stacks.push(VecDeque::new());
                    }

                    stacks[crate_i].push_front(char);
                }
            }
        }
    }

    panic!();
}

fn main() {
    let input = fs::read_to_string("./inputs/day-05.txt").unwrap();

    let mut stacks = parse_stacks(&input);

    lazy_static! {
        static ref MOVEMENT_REGEX: Regex = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    }

    for line in input
        .lines()
        .skip_while(|line| !MOVEMENT_REGEX.is_match(line))
    {
        if let Some(captures) = MOVEMENT_REGEX.captures(line) {
            let (amount, from, to) = (
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            );

            let mut being_moved = Vec::new();

            for _ in 0..amount {
                let from_stack = &mut stacks[from - 1];

                let back = from_stack.pop_back().unwrap();
                being_moved.push(back);
            }

            for char in being_moved.into_iter().rev() {
                let to_stack = &mut stacks[to - 1];

                to_stack.push_back(char);
            }
        } else {
            continue;
        }
    }

    let mut answer = Vec::new();
    for stack in stacks {
        answer.push(stack.back().unwrap().to_owned());
    }
    let answer = String::from_iter(answer.into_iter());
    println!("{answer}");
}
