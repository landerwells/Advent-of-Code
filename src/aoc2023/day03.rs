use crate::utils::parse_grid;
use aochelpers;
use indoc::indoc;

pub fn run() {
    let test_input = indoc!(
        "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."
    );
    let input = aochelpers::get_daily_input(3, 2023).unwrap();

    println!("Test: {}", part_one(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let grid = parse_grid(input);
    let mut answer = 0;

    let mut current_num: Vec<char> = Vec::new();
    let mut valid = false;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].is_numeric() {
                if !valid {
                    valid = is_valid_char(&grid, i, j);
                }
                current_num.push(grid[i][j]);
            } else {
                if valid {
                    let string: String = current_num.clone().into_iter().collect();
                    let int: i32 = string.parse().unwrap();
                    answer += int;
                }
                current_num.clear();
                valid = false;
            }
            if j == grid[i].len() - 1 {
                if valid {
                    let string: String = current_num.clone().into_iter().collect();
                    let int: i32 = string.parse().unwrap();
                    answer += int;
                }
                current_num.clear();
                valid = false;
            }
        }
    }
    answer
}

fn part_two(input: &String) -> i32 {
    // part_two should take in a grid, iterate through every character looking for '*'
    // if it finds a '*', it should check if there are two distinct numbers next to it
    // if not, do nothing and continue, if so, multiply the two numbers and add it to the answer
    let grid = parse_grid(input);

    let mut answer = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '*' {
                let (num1, num2) = distinct_numbers(&grid, i, j);
                answer += num1 * num2;
            }
        }
    }

    answer
}

fn distinct_numbers(grid: &Vec<Vec<char>>, row: usize, col: usize) -> (i32, i32) {
    let rows = grid.len();
    let cols = grid[0].len();

    // Define the king's move offsets
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut numbers: Vec<i32> = Vec::new();

    for &(offset_row, offset_col) in &offsets {
        let mut new_row = row as isize + offset_row;
        let mut new_col = col as isize + offset_col;

        // Check if the new indices are within bounds
        if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
            // Check if the character is not '.' and is numeric
            if grid[new_row as usize][new_col as usize] != '.' {
                let mut number_str = String::new();

                // Collect the entire numeric sequence on the right side
                while new_col < cols as isize
                    && grid[new_row as usize][new_col as usize].is_numeric()
                {
                    number_str.push(grid[new_row as usize][new_col as usize]);
                    new_col += 1;
                }

                // Reset indices for the left side
                new_row = row as isize + offset_row;
                new_col = col as isize + offset_col - 1;

                // Collect the entire numeric sequence on the left side
                while new_col >= 0 && grid[new_row as usize][new_col as usize].is_numeric() {
                    number_str.insert(0, grid[new_row as usize][new_col as usize]);
                    new_col -= 1;
                }

                // Parse the numeric sequence
                if let Ok(int) = number_str.parse::<i32>() {
                    // Check for duplicates before pushing into the vector
                    if !numbers.contains(&int) {
                        numbers.push(int);
                    }
                }
            }
        }
    }

    // Ensure there are at least two distinct numbers
    if numbers.len() >= 2 {
        (numbers[0], numbers[1])
    } else {
        // Return some default values or handle this case based on your requirements
        (0, 0)
    }
}

fn is_valid_char(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    // Define the king's move offsets
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for &(offset_row, offset_col) in &offsets {
        let new_row = row as isize + offset_row;
        let new_col = col as isize + offset_col;

        // Check if the new indices are within bounds
        if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
            // Check if the character is not '.' and not numeric
            if grid[new_row as usize][new_col as usize] != '.'
                && !grid[new_row as usize][new_col as usize].is_numeric()
            {
                return true;
            }
        }
    }

    false
}
