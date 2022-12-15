use std::{collections::HashSet, fs};
use util::{direction::Direction, point::Point, regex};

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

    let mut head_position: Point<i32> = Point::new(0, 0);
    let mut tail_position: Point<i32> = Point::new(0, 0);
    let mut tail_covered: HashSet<Point<i32>> = HashSet::from([tail_position]);

    for movement in input.0 {
        let (direction, distance) = movement;
        let offset = direction.to_offset();
        for _ in 0..distance {
            // Update head position
            head_position = Point::new(head_position.x + offset.x, head_position.y + offset.y);

            // Move tail to match if not touching
            let difference = Point::new(
                head_position.x - tail_position.x,
                head_position.y - tail_position.y,
            );

            if difference.x.abs() > 1 || difference.y.abs() > 1 {
                tail_position = Point::new(
                    tail_position.x + difference.x.signum(),
                    tail_position.y + difference.y.signum(),
                );
            }

            // Add the tail to the HashSet
            tail_covered.insert(tail_position);
        }
    }

    let tail_places = tail_covered.len();
    println!("{tail_places}");
}
