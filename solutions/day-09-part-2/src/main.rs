use std::{collections::HashSet, fs};
use util::{direction::Direction, point::Point, regex};

const ROPE_LENGTH: usize = 10;

struct Input(Vec<(Direction, i32)>);

fn parse_input(input: String) -> Input {
    Input(
        input
            .lines()
            .map(|line| {
                regex::lazy_static! {
                    static ref LINE_REGEX: regex::Regex = regex::Regex::new(r"(.) (\d*)").unwrap();
                }

                let captures = LINE_REGEX.captures(line).unwrap();

                let (direction, distance) = (
                    captures.get(1).unwrap().as_str(),
                    captures.get(2).unwrap().as_str(),
                );

                let direction = match direction {
                    "U" => Direction::Up,
                    "L" => Direction::Left,
                    "D" => Direction::Down,
                    "R" => Direction::Right,
                    _ => panic!(),
                };

                let distance = distance.parse::<i32>().unwrap();

                (direction, distance)
            })
            .collect(),
    )
}

fn main() {
    let input = parse_input(fs::read_to_string("./inputs/day-09.txt").unwrap());

    let mut rope_positions: Vec<Point<i32>> = Vec::from([Point::new(0, 0); ROPE_LENGTH]);

    let mut tail_covered: HashSet<Point<i32>> = HashSet::from([*rope_positions.last().unwrap()]);

    for movement in input.0 {
        let (direction, distance) = movement;
        let offset = direction.to_offset();
        for _ in 0..distance {
            // Update head position
            let head = rope_positions.first_mut().unwrap();
            *head = Point::new(head.x + offset.x, head.y + offset.y);

            // Update each positons based on the one before it
            for i in 1..rope_positions.len() {
                let prev_i = i - 1;

                let previous = rope_positions[prev_i];
                let current = rope_positions[i];

                let difference = Point::new(previous.x - current.x, previous.y - current.y);

                let new_current = if difference.x.abs() > 1 || difference.y.abs() > 1 {
                    Point::new(
                        current.x + difference.x.signum(),
                        current.y + difference.y.signum(),
                    )
                } else {
                    current
                };

                rope_positions[i] = new_current;
            }

            // Add the tail to the HashSet
            tail_covered.insert(*rope_positions.last().unwrap());
        }
    }

    let tail_places = tail_covered.len();
    println!("{tail_places}");
}
