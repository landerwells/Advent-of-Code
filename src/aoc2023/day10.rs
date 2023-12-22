use std::collections::VecDeque;
use std::env;
use aochelpers;

const LEFT: i32 = 3;
const RIGHT: i32 = 1;
const UP: i32 = 0;
const DOWN: i32 = 2;
type grid_point = (f64, f64);

pub fn run() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(10, 2023).unwrap();

    println!("Day Ten Answers:");
    println!("Part One: {}", solve(input.clone(), true));
    println!("Part Two: {}", solve(input, false));
}

fn solve(input: String, part_one: bool) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut points: Vec<grid_point> = Vec::new();
    let loop_length = finish_loop((43, 8), grid, UP, &mut points);
    if part_one {
        return loop_length / 2;
    }
    let area = interior_area(points);

    let num_interior_points = area as i32 - loop_length / 2 + 1;

    num_interior_points
}

fn interior_area(points: Vec<grid_point>) -> f64 {
    let padded_points = [&points[..], &points[0..1]].concat(); // form pair with last and first
    let area_sum: f64 = padded_points
        .iter()
        .zip(padded_points.iter().skip(1))
        .map(|((p1_row, p1_col), (p2_row, p2_col))| p1_row * p2_col - p2_row * p1_col)
        .sum();

    area_sum.abs() / 2.0
}

fn finish_loop(start_pos: (usize, usize), grid: Vec<Vec<char>>, start_direction: i32, points: &mut Vec<grid_point>) -> i32 {
    let mut stack: VecDeque<(usize, usize, i32, i32)> = VecDeque::new();
    stack.push_back((start_pos.0, start_pos.1, start_direction, 1));

    while let Some((row, col, previous_direction, count)) = stack.pop_back() {
        if grid[row][col] == 'S' {
            return count;
        }
        points.push((row as f64, col as f64));

        match grid[row][col] {
            '|' => {
                if previous_direction == UP {
                    stack.push_back((row + 1, col, UP, count + 1));
                } else {
                    stack.push_back((row - 1, col, DOWN, count + 1));
                }
            }
            '-' => {
                if previous_direction == LEFT {
                    stack.push_back((row, col + 1, LEFT, count + 1));
                } else {
                    stack.push_back((row, col - 1, RIGHT, count + 1));
                }
            }
            'L' => {
                if previous_direction == UP {
                    stack.push_back((row, col + 1, LEFT, count + 1));
                } else {
                    stack.push_back((row - 1, col, DOWN, count + 1));
                }
            }
            'J' => {
                if previous_direction == UP {
                    stack.push_back((row, col - 1, RIGHT, count + 1));
                } else {
                    stack.push_back((row - 1, col, DOWN, count + 1));
                }
            }
            '7' => {
                if previous_direction == LEFT {
                    stack.push_back((row + 1, col, UP, count + 1));
                } else {
                    stack.push_back((row, col - 1, RIGHT, count + 1));
                }
            }
            'F' => {
                if previous_direction == DOWN {
                    stack.push_back((row, col + 1, LEFT, count + 1));
                } else {
                    stack.push_back((row + 1, col, UP, count + 1));
                }
            }
            _ => {
                panic!("Illegal move")
            }
        }
    }
    // If 'S' is not reached, return a default value (e.g., -1)
    -1
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_solve_part_one() {
//         assert_eq!(solve_part_one("src/testinput.txt"), 8)
//     }

//     #[test]
//     fn test_finish_loop() {

//         let input: String = fs::read_to_string("src/testinput.txt".to_string()).unwrap();
//         let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

//         assert_eq!(finish_loop((3, 0), grid, UP, 1), 16);
//     }
// }
