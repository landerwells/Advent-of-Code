use aochelpers;
use indoc::indoc;
use regex::Regex;

pub fn run() {
    let test_input = indoc!("turn on 0,0 through 999,999");
    let input: String = aochelpers::get_daily_input(6, 2015).unwrap();

    println!("Test: {}", part_one(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let mut grid = vec![vec![false; 1000]; 1000];

    for line in lines {
        light_action(&line, &mut grid);
    }

    count_lights(&grid)
}

fn part_two(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut grid = vec![vec![0; 1000]; 1000];

    for line in lines {
        brightness(&line, &mut grid);
    }

    count_brightness(&grid)
}

fn count_brightness(grid: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            count += grid[x][y];
        }
    }

    count
}

fn brightness(line: &str, grid: &mut Vec<Vec<i32>>) {
    let re = Regex::new(r"(on|off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let caps = re.captures(line).unwrap();

    let action = caps.get(1).unwrap().as_str();

    let start_x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let start_y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let end_x = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let end_y = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

    for x in start_x..=end_x {
        for y in start_y..=end_y {
            match action {
                "on" => grid[x][y] += 1,
                "off" => {
                    if grid[x][y] != 0 {
                        grid[x][y] -= 1;
                    }
                }
                "toggle" => grid[x][y] += 2,
                _ => panic!("Unknown action: {}", action),
            }
        }
    }
}

fn light_action(line: &str, grid: &mut Vec<Vec<bool>>) {
    let re = Regex::new(r"(on|off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let caps = re.captures(line).unwrap();

    let action = caps.get(1).unwrap().as_str();

    let start_x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let start_y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let end_x = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let end_y = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

    for x in start_x..=end_x {
        for y in start_y..=end_y {
            match action {
                "on" => grid[x][y] = true,
                "off" => grid[x][y] = false,
                "toggle" => grid[x][y] = !grid[x][y],
                _ => panic!("Unknown action: {}", action),
            }
        }
    }
}

fn count_lights(grid: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] {
                count += 1;
            }
        }
    }

    count
}
