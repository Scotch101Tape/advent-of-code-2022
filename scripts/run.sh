# This script runs the specified day and part code

if [ -z "$1" ]
then
    echo "What day number?"
    read passed_day_num
else
    passed_day_num=$1
fi

if [ -z "$2" ]
then
    echo "What part number?"
    read part_num
else
    day_num=$2
fi

# Pad the day number
if [ ${#1} -gt 1 ]
then
    day_num=$passed_day_num
else
    day_num=0$passed_day_num
fi

cargo run --bin day-$day_num-part-$part_num
