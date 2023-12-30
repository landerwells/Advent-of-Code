use aochelpers;
use std::collections::HashSet;
use std::env;

pub fn run() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(4, 2023).unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();

    println!("Day Four Answers:");
    println!("Part One: {}", solve_part_one(lines.clone()));
    println!("Part Two: {}", solve_part_two(lines));
}

fn solve_part_one(lines: Vec<String>) -> i32 {
    let mut answer = 0;
    for line in &lines {
        answer += line_points(line.clone());
    }
    answer
}

// This method is so god damn slow
fn solve_part_two(lines: Vec<String>) -> i32 {
    // Reading in file
    let mut line_number = 0;
    let mut total_count = 0;
    let mut winning_numbers_vector: Vec<i32> = Vec::new();

    for line in &lines {
        winning_numbers_vector.push(winning_numbers(line.clone()));
    }

    // Vector to count how many cards total we have
    let mut cards: Vec<i32> = vec![1; lines.len()];
    for line in &lines {
        while cards[line_number] != 0 {
            let current_points: usize = winning_numbers_vector[line_number] as usize;
            for i in 0..current_points {
                cards[line_number + i + 1] += 1;
            }
            cards[line_number] -= 1;
            total_count += 1;
        }
        line_number += 1;
    }
    total_count
}

fn line_points(line: String) -> i32 {
    let mut current_points = 0;

    for _i in 0..winning_numbers(line) {
        if current_points == 0 {
            current_points += 1;
        } else {
            current_points *= 2;
        }
    }
    current_points
}

fn winning_numbers(line: String) -> i32 {
    let mut set = HashSet::new();
    let mut current_points = 0;
    let mut first_half = true;
    let parts: Vec<&str> = line.split_whitespace().collect();

    for part in parts {
        if part == "|" {
            first_half = false;
        }
        if first_half {
            set.insert(part);
        } else {
            if set.contains(part) {
                current_points += 1;
            }
        }
    }
    current_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        // assert_eq!(13, solve_part_one("src/input2.txt".to_string()));
    }

    #[test]
    fn test_solve_part_two() {
        // assert_eq!(30, solve_part_two("src/input2.txt".to_string()));
    }

    #[test]
    fn test_line_points() {
        assert_eq!(
            8,
            line_points("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string())
        );
    }
    #[test]
    fn test_winning_numbers() {
        assert_eq!(
            4,
            winning_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string())
        );
    }
}
