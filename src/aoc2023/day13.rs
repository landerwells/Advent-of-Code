use aochelpers;

pub fn run() {
    let input = aochelpers::get_daily_input(13, 2023).unwrap();
    println!("Day Thirteen Answers:");
    println!("");
    println!("Part One: {}", solve_part_one(input));
}

fn solve_part_one(input: String) -> i32 {
    let blocks: Vec<String> = input.split("\n\n").map(String::from).collect();

    let mut sum = 0;
    for i in blocks {
        let block: Vec<Vec<char>> = i.lines().map(|line| line.chars().collect()).collect();
        sum += find_reflection(block);
    }
    sum
}

fn find_reflection(block: Vec<Vec<char>>) -> i32 {
    let mut row_stack: Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    let mut entering: bool = true;
    let mut valid: bool = true;
    for line in block.clone() {
        if row_stack.is_empty() && i == 0 {
            row_stack.push(line);
            i += 1;
        } else if entering {
            if row_stack.last().unwrap() == &line {
                entering = false;
                row_stack.pop();
            } else {
                row_stack.push(line);
                i += 1;
            }
        } else {
            if row_stack.is_empty() {
                break;
            }
            if row_stack.pop() != Some(line) {
                valid = false;
            }
        }
    }

    if valid && !entering {
        return i * 100;
    }

    entering = true;
    valid = true;
    i = 0;
    row_stack.clear();

    for j in 0..block[0].len() {
        let column_data: Vec<char> = block.iter().map(|row| row[j]).collect();
        if row_stack.is_empty() && i == 0 {
            row_stack.push(column_data);
            i += 1;
        } else if entering {
            if row_stack.last().unwrap() == &column_data {
                entering = false;
                row_stack.pop();
            } else {
                row_stack.push(column_data);
                i += 1;
            }
        } else {
            if row_stack.is_empty() {
                break;
            }
            if row_stack.pop() != Some(column_data) {
                valid = false;
            }
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_mirror_one() {
        let input: String = "....#....##..##..
##.#.....#....#..
....##...#.##.#..
..##..#####..####
###..##..######..
.....##.#.#..#.#.
.#.#####.##..##.#
###.###...####...
....####.#....#.#"
            .to_string();

        let block: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(12, find_reflection(block));
    }

    #[test]
    fn test_horizontal_mirror() {
        let input: String = ".#...###...
.....##.###
..#..##.#..
####.......
..##..#.##.
######.###.
########..#
########..#
######.###.
..##..#.##.
####......."
            .to_string();

        let block: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(700, find_reflection(block));
    }

    #[test]
    fn test_vertical_mirror() {
        let input: String = "#.##.#.####.#
.#..#..#..#..
.####.#....#.
#....#......#
.#..#.#....#.
.##.#.######.
#....########
#....##....##
########..###
.#..#..####..
.#..#..####..
..##....##...
.####...##...
.#..#.##..##.
##..#########"
            .to_string();

        let block: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(9, find_reflection(block));
    }
}
