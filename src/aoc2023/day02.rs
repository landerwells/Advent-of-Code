use aochelpers;
use std::env;
use regex::Regex;

pub fn run() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(2, 2023).unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();

    println!("Day Two Answers:");
    println!("Part One: {}", solve(lines.clone()));
    println!("Part Two: {}", sum_power_sets(lines.clone()));
}

fn solve(lines: Vec<String>) -> i32 {

    let mut answer = 0;
    let mut game_number = 1;

    for line in lines {
        if is_valid_game(line) {
            answer += game_number;
        }
        game_number += 1;
    }
    answer
}

fn is_valid_game(mut line: String) -> bool {
    let offset = line.find(':').unwrap_or(line.len());
    line.drain(..offset + 2);

    // Removed the slashes and the "g" flag
    let re = Regex::new(r"(?P<digit>\d+) (?P<color>blue|red|green)").unwrap();

    for captures in re.captures_iter(&line) {
        // Extract the color into a variable
        let color = captures["color"].to_string();

        match color.as_str() {
            "red" => {
                if captures["digit"].parse::<i32>().unwrap() > 12 {
                    return false;
                }
            }
            "blue" => {
                if captures["digit"].parse::<i32>().unwrap() > 14 {
                    return false;
                }
            }
            "green" => {
                if captures["digit"].parse::<i32>().unwrap() > 13 {
                    return false;
                }
            }
            _ => {
                // Handle other colors or unexpected cases
            }
        }
    }
    true
}

fn sum_power_sets(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        sum += find_power_set(line);
    }
    sum
}

fn find_power_set(mut line: String) -> i32 {

    let offset = line.find(':').unwrap_or(line.len());
    line.drain(..offset + 2);

    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    // Removed the slashes and the "g" flag
    let re = Regex::new(r"(?P<digit>\d+) (?P<color>blue|red|green)").unwrap();

    for captures in re.captures_iter(&line) {
        // Extract the color into a variable
        let color = captures["color"].to_string();
        let digit = captures["digit"].parse::<i32>().unwrap();

        match color.as_str() {
            "red" => {
                if digit > max_red {
                    max_red = digit;
                }
            }
            "blue" => {
                if digit > max_blue {
                    max_blue = digit;
                }
            }
            "green" => {
                if digit > max_green {
                    max_green = digit;
                }
            }
            _ => {
                // Handle other colors or unexpected cases
            }
        }
    }

    let power_set = max_blue * max_red * max_green;

    power_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let lines: Vec<String> = input.lines().map(String::from).collect();

        let answer = solve(lines);

        assert_eq!(answer, 8);
    }

    #[test]
    fn test_is_game_valid() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        assert_eq!(false, is_valid_game(input.to_string()));
    }

    #[test]
    fn test_sum_power_sets() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let lines: Vec<String> = input.lines().map(String::from).collect();

        assert_eq!(sum_power_sets(lines.clone()), 2286);
    }
}
