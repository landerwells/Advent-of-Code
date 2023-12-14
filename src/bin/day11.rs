use aochelpers;
use std::fs;
use std::env;

fn main() {
    let input = aochelpers::get_daily_input(11, 2023).unwrap();
    println!("Day Eleven Answers:");
    println!("Part One: {}", solve_part_one(input));
    // println!("Part One: {}", solve_part_one("src/input"))
}

fn solve_part_one(input: String) -> i32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    double_rows_and_columns(&mut grid);

    let mut hash_coordinates = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                hash_coordinates.push((i, j));
            }
        }
    }

    let mut shortest_distance = f64::INFINITY;

    for &coord1 in hash_coordinates.iter() {
        for &coord2 in hash_coordinates.iter() {
            if coord1 != coord2 {
                let distance = calculate_distance(coord1, coord2);
                shortest_distance += distance;
            }
        }
    }

    shortest_distance as i32
}


fn calculate_distance(coord1: (usize, usize), coord2: (usize, usize)) -> f64 {
    let row_diff = num::abs(coord1.0 as f64 - coord2.0 as f64);
    let col_diff = num::abs(coord1.1 as f64 - coord2.1 as f64);

    row_diff + col_diff
}

fn double_rows_and_columns(grid: &mut Vec<Vec<char>>) {
    let mut rows_to_double = Vec::new();
    let mut cols_to_double = Vec::new();

    // Check rows
    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            rows_to_double.push(i);
        }
    }

    // Check columns
    for col in 0..grid[0].len() {
        if grid.iter().all(|row| row[col] == '.') {
            cols_to_double.push(col);
        }
    }

    // Double rows
    for &row_index in rows_to_double.iter().rev() {
        let row = grid[row_index].clone();
        grid.insert(row_index, row);
    }

    // Double columns
    for &col_index in cols_to_double.iter().rev() {
        for row in grid.iter_mut() {
            row.insert(col_index, '.');
        }
    }
}

fn expand_universe(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut temp: Vec<Vec<char>> = Vec::new();

    for row in grid {
        if row.contains(&'#') {
            temp.push(row.clone());
            temp.push(row.clone());
        }
        temp.push(row.clone());

    }

    temp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calulate_distance() {
        assert_eq!(calculate_distance((1, 6), (5, 11)), 9.0);
    }

    #[test]
    fn test_expand_universe() {
        let input_one = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let input_two = "....#........
        .........#...
        #............
        .............
        .............
        ........#....
        .#...........
        ............#
        .............
        .............
        .........#...
        #....#.......";

        let mut grid_one: Vec<Vec<char>> = input_one.lines().map(|line| line.chars().collect()).collect();
        let grid_two: Vec<Vec<char>> = input_two.lines().map(|line| line.chars().collect()).collect();

        assert_eq!(expand_universe(grid_one), grid_two);
    }
}