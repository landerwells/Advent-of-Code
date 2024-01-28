use aochelpers;
use indoc::indoc;
use std::collections::HashSet;

// Could definitely improve the speed on this day
pub fn run() {
    let test_input = indoc!(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    );
    let input = aochelpers::get_daily_input(4, 2023).unwrap();

    println!("Test: {}", part_one(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut answer = 0;
    for line in &lines {
        answer += line_points(line.clone());
    }
    answer
}

// This method is so god damn slow
fn part_two(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut line_number = 0;
    let mut total_count = 0;
    let mut winning_numbers_vector: Vec<i32> = Vec::new();

    for line in &lines {
        winning_numbers_vector.push(winning_numbers(line.clone()));
    }

    // Vector to count how many cards total we have
    let mut cards: Vec<i32> = vec![1; lines.len()];
    for _line in &lines {
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
        } else if set.contains(part) {
            current_points += 1;
        }
    }
    current_points
}
