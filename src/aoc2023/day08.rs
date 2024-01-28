use aochelpers;
use indoc::indoc;
use num::integer::lcm;
use std::collections::HashMap;

pub fn run() {
    let test_input = indoc!(
        "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)"
    );
    let input = aochelpers::get_daily_input(8, 2023).unwrap();

    println!("Test: {}", part_one(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();
    let directions = lines.remove(0);
    lines.remove(0);

    let mut next_key = "AAA".to_string();
    let last_key = "ZZZ".to_string();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in &lines {
        // make parts only contain alphabetic characters of line split by whitespace into a vec
        let cleaned_string: String = line
            .chars()
            .filter(|&c| c.is_alphabetic() || c.is_whitespace())
            .collect();
        let parts: Vec<String> = cleaned_string
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        map.insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
    }
    let mut count = 0;

    loop {
        let c = directions
            .chars()
            .nth(count as usize % directions.len())
            .unwrap();
        let left_right = map.get(&next_key).unwrap();
        if c == 'L' {
            next_key = left_right.0.clone();
        } else if c == 'R' {
            next_key = left_right.1.clone();
        }
        count += 1;
        if next_key == last_key {
            break;
        }
    }
    count
}

fn part_two(input: &String) -> i128 {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();
    let directions = lines.remove(0);
    lines.remove(0);

    let starts: Vec<String> = lines
        .iter()
        .filter_map(|line| {
            let first_word = line.split_whitespace().nth(0)?;
            if first_word.chars().last() == Some('A') {
                Some(first_word.to_string())
            } else {
                None
            }
        })
        .collect();

    let ends: Vec<String> = lines
        .iter()
        .filter_map(|line| {
            let first_word = line.split_whitespace().nth(0)?;
            if first_word.chars().last() == Some('Z') {
                Some(first_word.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut counts: Vec<i128> = Vec::new();

    for mut next_key in starts {
        for line in &lines {
            // make parts only contain alphabetic characters of line split by whitespace into a vec
            let cleaned_string: String = line
                .chars()
                .filter(|&c| c.is_alphabetic() || c.is_whitespace())
                .collect();
            let parts: Vec<String> = cleaned_string
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            map.insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
        }
        let mut count = 0;

        loop {
            let c = directions
                .chars()
                .nth(count as usize % directions.len())
                .unwrap();
            let left_right = map.get(&next_key).unwrap();
            if c == 'L' {
                next_key = left_right.0.clone();
            } else if c == 'R' {
                next_key = left_right.1.clone();
            }
            count += 1;
            if ends.contains(&next_key) {
                break;
            }
        }
        counts.push(count);
    }
    // find and return lcm of counts
    counts.iter().fold(1, |acc, &x| lcm(acc, x))
}
