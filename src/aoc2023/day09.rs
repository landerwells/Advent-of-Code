use std::env;
use aochelpers;


pub fn run() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(9, 2023).unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();

    println!("Day Nine Answers:");
    println!("Part One: {}", solve_part_one(lines.clone(), true));
    println!("Part Two: {}", solve_part_one(lines, false));
}

fn solve_part_one(lines: Vec<String>, part_one: bool) -> i32 {
    let mut sum: i32 = 0;

    for line in &lines {
        let mut parts: Vec<i32> = line.split_whitespace().collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if !part_one {
            parts.reverse();
        }

        let mut last_digits: Vec<i32> = Vec::new();
        last_digits.push(parts[parts.len() - 1]);

        // build difference from the difference between each element in parts
        let mut difference: Vec<i32> = parts.iter().zip(parts.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();

        last_digits.push(difference[difference.len() - 1]);

        // while all digits in difference are not 0
        while difference.iter().any(|&x| x != 0) {
            difference = difference.iter().zip(difference.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
            last_digits.push(difference[difference.len() - 1]);
        }
        sum += last_digits.iter().fold(0, |acc, x| acc + x);
    }
    sum
}
