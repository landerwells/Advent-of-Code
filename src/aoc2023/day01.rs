use aochelpers;
use std::env;

pub fn run() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(1, 2023).unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();
    // let answer_two = solve(lines.iter().map(|l| l.as_str()).collect());

    println!("Day One Answers:");
    println!("Part One: {}", solve_part_one(lines.clone()));
    println!("Part Two: {}", solve_part_two(input));
}

pub fn solve_part_two(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| {
            line.to_string()
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| {
                    c.to_digit(10)
                        .expect("Failed to convert character to digit")
                })
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            10 * vec.first().expect("Every line must have atleast one digit")
                + vec.last().expect("Every line must have atleast one digit")
        })
        .sum()
}

fn solve_part_one(lines: Vec<String>) -> i32 {

    let mut sum: i32 = 0;

    for line in lines {
        let numbers: String = line.chars().filter(|&c| c.is_numeric()).collect();
        let first = numbers.chars().next().unwrap();
        let last = numbers.chars().last().unwrap();
        let result = format!("{}{}", first, last);
        let result_int: i32 = result.parse().unwrap();
        sum += result_int;
    }
    sum
}
