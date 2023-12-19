use aochelpers;

fn main() {
    let input = aochelpers::get_daily_input(11, 2023).unwrap();
    println!("Day Eleven Answers:");
    println!("Part One: {}", solve_part_one(input.clone(), 2));
    println!("Part Two: {}", solve_part_one(input, 1000000))
}

fn solve_part_one(input: String, multiple: i32) -> i64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut hash_coordinates = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                hash_coordinates.push((i, j));
            }
        }
    }

    let mut rows_to_add: Vec<usize> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        if row.contains(&'#') {
            continue;
        } else {
            rows_to_add.push(i);
        }
    }

    let mut columns_to_add: Vec<usize> = Vec::new();

    for i in 0..grid[0].len() {
        let column_data: Vec<char> = grid.iter().map(|row| row[i]).collect();
        if column_data.contains(&'#') {
            continue;
        } else {
            columns_to_add.push(i);
        }
    }

    let mut shortest_distance: f64 = 0.0;

    // if there are rows or columns to be added, add the distance to the coordinate so that the distance is calculated correctly

    for &coord1 in hash_coordinates.iter() {
        for &coord2 in hash_coordinates.iter() {
            if coord1 != coord2 {
                // check if there are any rows_to_add and add the multiple in between the x coord
                // check if there are any columns_to_add and add the multiple in between the y coord
                let mut temp_coord1 = coord1;
                let mut temp_coord2 = coord2;

                if !rows_to_add.is_empty() {
                    for i in &rows_to_add {
                        if i < &coord1.0 && i > &coord2.0  {
                            temp_coord1.0 += multiple as usize - 1;
                        } else if i > &coord1.0 && i < &coord2.0 {
                            temp_coord2.0 += multiple as usize - 1;
                        }
                    }
                }

                if !columns_to_add.is_empty() {
                    for i in &columns_to_add {
                        if i < &coord1.1 && i > &coord2.1  {
                            temp_coord1.1 += multiple as usize - 1;
                        } else if i > &coord1.1 && i < &coord2.1 {
                            temp_coord2.1 += multiple as usize - 1;
                        }
                    }
                }

                let distance = calculate_distance(temp_coord1, temp_coord2);
                shortest_distance += distance;

            }
        }
    }

    shortest_distance as i64 / 2
}

fn calculate_distance(coord1: (usize, usize), coord2: (usize, usize)) -> f64 {
    let row_diff = num::abs(coord1.0 as f64 - coord2.0 as f64);
    let col_diff = num::abs(coord1.1 as f64 - coord2.1 as f64);

    row_diff + col_diff
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distance((1, 6), (5, 11)), 9.0);
    }


    #[test]
    fn test_solve_part_one() {

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

        assert_eq!(solve_part_one(input_one.to_string(), 10), 1030);
        assert_eq!(solve_part_one(input_one.to_string(), 100), 8410);


    }
}
