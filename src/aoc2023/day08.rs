use std::collections::HashMap;
use std::env;
use aochelpers;
use num::integer::lcm;

pub fn run() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(8, 2023).unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();

    println!("Part One: {}", solve_part_one(lines.clone()));
    println!("Part Two: {}", solve_part_two(lines.clone()));
}

fn solve_part_one(mut lines: Vec<String>) -> i32 {

    let directions = lines.remove(0);
    lines.remove(0);

    let mut next_key = "AAA".to_string();
    let last_key = "ZZZ".to_string();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in &lines {
        // make parts only contain alphabetic characters of line split by whitespace into a vec
        let cleaned_string: String = line.chars().filter(|&c| c.is_alphabetic() || c.is_whitespace()).collect();
        let parts: Vec<String> = cleaned_string.split_whitespace().map(|s| s.to_string()).collect();
        map.insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
    }
    let mut count = 0;

    loop {
        let c = directions.chars().nth(count as usize % directions.len()).unwrap();
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

fn solve_part_two(mut lines: Vec<String>) -> i128 {

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
            let cleaned_string: String = line.chars().filter(|&c| c.is_alphabetic() || c.is_whitespace()).collect();
            let parts: Vec<String> = cleaned_string.split_whitespace().map(|s| s.to_string()).collect();
            map.insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
        }
        let mut count = 0;

        loop {
            let c = directions.chars().nth(count as usize % directions.len()).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        assert_eq!(6, solve_part_one("testinput.txt"));
        assert_eq!(2, solve_part_one("testinput2.txt"));
    }
}
