use std::{collections::HashSet, fs};

#[derive(Hash, PartialEq, PartialOrd, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let input = fs::read_to_string("./inputs/day-08.txt").unwrap();

    let numbers = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut trees_visible: HashSet<Point> = HashSet::new();

    for x in 0..numbers.len() {
        let mut max = -1;
        for y in 0..numbers[0].len() {
            let number = numbers[x][y] as i32;
            if number > max {
                trees_visible.insert(Point { x, y });
                max = number;
            }
        }
    }

    for x in 0..numbers.len() {
        let mut max = -1;
        for y in (0..numbers[0].len()).rev() {
            let number = numbers[x][y] as i32;
            if number > max {
                trees_visible.insert(Point { x, y });
                max = number;
            }
        }
    }

    for y in 0..numbers[0].len() {
        let mut max = -1;
        for x in 0..numbers.len() {
            let number = numbers[x][y] as i32;
            if number > max {
                trees_visible.insert(Point { x, y });
                max = number;
            }
        }
    }

    for y in 0..numbers[0].len() {
        let mut max = -1;
        for x in (0..numbers.len()).rev() {
            let number = numbers[x][y] as i32;
            if number > max {
                trees_visible.insert(Point { x, y });
                max = number;
            }
        }
    }

    let amount_trees_visible = trees_visible.len();
    println!("{amount_trees_visible}");
}
