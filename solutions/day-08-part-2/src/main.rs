use std::{fs, iter};

#[derive(Hash, PartialEq, PartialOrd, Eq)]
struct Point {
    x: usize,
    y: usize,
}

static OFFSETS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

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

    let best_score = numbers
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, number)| {
                    OFFSETS
                        .iter()
                        .map(|offset| {
                            let mut view = (x as i32, y as i32);
                            let mut hit_edge_of_forest = false;
                            let line_of_sight = iter::from_fn(|| {
                                view = (view.0 + offset.0, view.1 + offset.1);

                                if let Some(view_row) = numbers.get(view.0 as usize) {
                                    if let Some(number_at_view) = view_row.get(view.1 as usize) {
                                        if number_at_view < number {
                                            Some(())
                                        } else {
                                            None
                                        }
                                    } else {
                                        hit_edge_of_forest = true;
                                        None
                                    }
                                } else {
                                    hit_edge_of_forest = true;
                                    None
                                }
                            });
                            line_of_sight.count() + if hit_edge_of_forest { 0 } else { 1 }
                        })
                        .product::<usize>()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{best_score}");
}
