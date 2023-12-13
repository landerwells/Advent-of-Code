
use std::fs;

fn main() {
    println!("Part One: {}", solve_part_one("src/input"))
}

fn solve_part_one(filename: &str) -> i32 {
    let input = fs::read_to_string(filename.to_string()).unwrap();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    grid = expand_universe(grid);






    0
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
    fn test_expand_universe() {
        let input = fs::read_to_string("src/testinput".to_string()).unwrap();
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();


    }
}
