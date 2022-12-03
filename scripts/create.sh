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

new_cargo_toml=$(sed 's/^]/    ".\/solutions\/day-'$day_num'-part-1",\n    ".\/solutions\/day-'$day_num'-part-2",\n]/' cargo.toml)
echo "$new_cargo_toml" > cargo.toml

cargo new ./solutions/day-$day_num-part-1 --vcs none
cargo new ./solutions/day-$day_num-part-2 --vcs none

init_rust_code='use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day-'$day_num'.txt").unwrap();
}
'

echo "$init_rust_code" > ./solutions/day-$day_num-part-1/src/main.rs
echo "$init_rust_code" > ./solutions/day-$day_num-part-2/src/main.rs
