# This script creates an Advent of Code directory for the day specified

if [ -z "$1" ]
then
    echo "What day number?"
    read passed_day_num
else
    passed_day_num=$1
fi

# Pad the day number
if [ ${#1} -gt 1 ]
then
    day_num=$passed_day_num
else
    day_num=0$passed_day_num
fi

touch ./inputs/day-$day_num.txt

new_cargo_toml=$(sed 's/^]/    ".\/solutions\/day-'$day_num'-part-1",\n    ".\/solutions\/day-'$day_num'-part-2",\n]/' Cargo.toml)
echo "$new_cargo_toml" > Cargo.toml

cargo new ./solutions/day-$day_num-part-1 --vcs none
cargo new ./solutions/day-$day_num-part-2 --vcs none

init_rust_code='use std::fs;
use util;

fn main() {
    let input = fs::read_to_string("./inputs/day-'$day_num'.txt").unwrap();
}
'

init_cargo_code_part_1='[package]
name = "day-'$day_num'-part-1"
version = "0.1.0"
edition = "2021"

[dependencies]
util = { path = "../../util" }
'

init_cargo_code_part_2='[package]
name = "day-'$day_num'-part-2"
version = "0.1.0"
edition = "2021"

[dependencies]
util = { path = "../../util" }
'

echo "$init_rust_code" > ./solutions/day-$day_num-part-1/src/main.rs
echo "$init_rust_code" > ./solutions/day-$day_num-part-2/src/main.rs

echo "$init_cargo_code_part_1" > ./solutions/day-$day_num-part-1/Cargo.toml
echo "$init_cargo_code_part_2" > ./solutions/day-$day_num-part-2/Cargo.toml
