use std::fs;
use util::marker::find_marker;

fn main() {
    let input = fs::read_to_string("./inputs/day-06.txt").unwrap();
    let marker = find_marker(input.trim_end().as_bytes(), 4).expect("no marker found");
    println!("{marker}");
}
