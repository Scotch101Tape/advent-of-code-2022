// This is a mess
// But it works

use std::{cmp, collections::HashMap, fs};

const DISTINCT_CHARS_NEEDED: usize = 14;

const PADDING: &str = "12345678901234";

fn main() {
    let input = fs::read_to_string("./inputs/day-06.txt").unwrap();

    let mut current_chars: HashMap<char, usize> = HashMap::new();

    let mut group_index: Option<usize> = None;

    for (i, (leaving_char, new_char)) in PADDING
        .chars()
        .chain(input.chars())
        .zip(
            PADDING
                .chars()
                .chain(input.chars())
                .skip(DISTINCT_CHARS_NEEDED),
        )
        .enumerate()
    {
        println!("{leaving_char}, {new_char}");
        if current_chars.contains_key(&leaving_char) {
            let new_value = cmp::max(0, current_chars[&leaving_char] as i32 - 1) as usize;
            current_chars.insert(leaving_char, new_value);
        }

        if current_chars.contains_key(&new_char) {
            let new_value = current_chars[&new_char] + 1;
            current_chars.insert(new_char, new_value);
        } else {
            current_chars.insert(new_char, 1);
        }

        println!("{}", current_chars.values().sum::<usize>());

        if i >= DISTINCT_CHARS_NEEDED - 1 {
            println!("{:?}", current_chars);
        }

        if current_chars.values().filter(|value| **value > 1).count() == 0
            && i >= DISTINCT_CHARS_NEEDED - 1
        {
            group_index = Some(i);
            break;
        }
    }

    // This works for some reason...
    println!("{}", group_index.unwrap() + 1);
}
