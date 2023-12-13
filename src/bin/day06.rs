use std::env;
use aochelpers;

fn main() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(6, 2023).unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();

    // part one uses the regular input
    println!("Part One: {}", solve_part_one(lines.clone(), false));
    // part two removes the space between the numbers in the regular input
    // println!("Part Two: {}", solve_part_one(lines.clone(), true));
}

fn solve_part_one(lines: Vec<String>, part_two: bool) -> i128 {

    let times: Vec<i128> = lines[0].split_whitespace().skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<i128> = lines[1].split_whitespace().skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut current_ways = 0;
    let mut total_ways = 1;

    for time in 0..times.len() {
        for i in 0..times[time] {
            let rev = times[time] - i;
            if (rev * i) > distances[time] {
                current_ways += 1;
            }
        }
        total_ways *= current_ways;
        current_ways = 0;
    }

    total_ways

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_solve_part_one() {
    //     assert_eq!(288, solve_part_one("src/testinput.txt".to_string()));
    // }
}
