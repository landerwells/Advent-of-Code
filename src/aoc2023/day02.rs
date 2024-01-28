use aochelpers;
use indoc::indoc;
use regex::Regex;

pub fn run() {
    let test_input = indoc!(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    );
    let input = aochelpers::get_daily_input(2, 2023).unwrap();

    println!("Test: {}", part_one(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
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

fn part_two(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
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
