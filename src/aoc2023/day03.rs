use aochelpers;

pub fn run() {
    let input = aochelpers::get_daily_input(3, 2023).unwrap();
    let lines: Vec<String> = input.lines().map(String::from).collect();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in &lines {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    let answer = solve_part_one(&grid);
    let answer_two = solve_part_two(&grid);
    println!("Day Three Answers:");
    println!("");
    println!("Part One: {}", answer);
    println!("Part Two: {}", answer_two);
}

fn solve_part_one(grid: &Vec<Vec<char>>) -> i32 {
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

fn solve_part_two(grid: &Vec<Vec<char>>) -> i32 {
    // solve_part_two should take in a grid, iterate through every character looking for '*'
    // if it finds a '*', it should check if there are two distinct numbers next to it
    // if not, do nothing and continue, if so, multiply the two numbers and add it to the answer

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

// is_valid_pair should return true if a '*' is the symbol that is not a number,
// and there is another number next to the star that isn't the same number

// fn is_valid_pair(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
//     let rows = grid.len();
//     let cols = grid[0].len();

//     // Define the king's move offsets
//     let offsets = [
//         (-1, -1), (-1, 0), (-1, 1),
//         (0, -1),           (0, 1),
//         (1, -1),  (1, 0),  (1, 1),
//     ];

//     for &(offset_row, offset_col) in &offsets {
//         let new_row = row as isize + offset_row;
//         let new_col = col as isize + offset_col;

//         // Check if the new indices are within bounds
//         if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
//             // Check if the character is not '.' and not numeric
//             if grid[new_row as usize][new_col as usize] != '.' &&
//                !grid[new_row as usize][new_col as usize].is_numeric() {
//                 // Check if the character is not the same as the current character
//                 if grid[new_row as usize][new_col as usize] != grid[row][col] {
//                     return true;
//                 }
//             }
//         }
//     }

//     false
// }

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_solve_part_one() {

//         let input = fs::read_to_string("src/input2.txt").unwrap();

//         let lines: Vec<String> = input.lines().map(String::from).collect();

//         let mut grid: Vec<Vec<char>> = Vec::new();

//         for line in &lines {
//             let chars: Vec<char> = line.chars().collect();
//             grid.push(chars);
//         }

//         let answer = solve_part_one(&grid);

//         assert_eq!(4361, answer);
//     }

//     #[test]
//     fn test_solve_part_two() {

//         let input = fs::read_to_string("src/input2.txt").unwrap();

//         let lines: Vec<String> = input.lines().map(String::from).collect();

//         let mut grid: Vec<Vec<char>> = Vec::new();

//         for line in &lines {
//             let chars: Vec<char> = line.chars().collect();
//             grid.push(chars);
//         }

//         let answer = solve_part_two(&grid);

//         assert_eq!(467835, answer);
//     }

//     #[test]
//     fn test_is_valid_char() {

//         let input = fs::read_to_string("src/input.txt").unwrap();

//         let lines: Vec<String> = input.lines().map(String::from).collect();

//         let mut grid: Vec<Vec<char>> = Vec::new();

//         for line in &lines {
//             let chars: Vec<char> = line.chars().collect();
//             grid.push(chars);
//         }

//         let answer = is_valid_char(&grid, 131,44);

//         assert_eq!(false, answer);
//     }

//     #[test]
//     fn test_distinct_numbers() {

//         let input = fs::read_to_string("src/input2.txt").unwrap();
//         let lines: Vec<String> = input.lines().map(String::from).collect();
//         let mut grid: Vec<Vec<char>> = Vec::new();

//         for line in &lines {
//             let chars: Vec<char> = line.chars().collect();
//             grid.push(chars);
//         }

//         assert_eq!((467, 35), distinct_numbers(&grid, 1, 3));
//     }
// }
