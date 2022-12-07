use itertools::izip;
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day-06.txt").unwrap();

    let input_length = input.len();

    let group_index = 1 + input_length
        - izip!(
            input.chars().skip(0),
            input.chars().skip(1),
            input.chars().skip(2),
            input.chars().skip(3)
        )
        .skip_while(|chars| {
            let chars = chars.to_owned();

            !(chars.0 != chars.1
                && chars.0 != chars.2
                && chars.0 != chars.3
                && chars.1 != chars.2
                && chars.1 != chars.3
                && chars.2 != chars.3)
        })
        .count();

    println!("{}", group_index);
}
