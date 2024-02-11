#!/bin/sh

# The script expects one argument: the day number
year=$1
day_number=$2

# Use printf to format the day number with leading zeros
formatted_day=$(printf "day%02d" "$day_number")


# copy the template file into the correct year and name it the correct day, 
# and modify the the contents to match

mkdir -p src/aoc$1
cp src/template.rs src/aoc$1/$formatted_day.rs


# sed 's/get_daily_input\(\)/get_daily_input\(day_number, year\)/' src/aoc$1/$formatted_day
sed -i '' "s/get_daily_input()/get_daily_input($day_number, $year)/" src/aoc$year/$formatted_day.rs


# the last things to do are modify the mod file in the specific year and the main file


# Append the new module to the mod.rs file
mod_file="src/aoc$year/mod.rs"

# Add new mod declaration
echo "pub mod $formatted_day;" >> $mod_file

# Add the day's run function call in the run_all function. This part is a bit tricky
# because we need to insert the line before the last brace of the run_all function.
# We can use sed to do this.

sed -i '' "/run_all()/ a\\
    $formatted_day::run();\\
" $mod_file

