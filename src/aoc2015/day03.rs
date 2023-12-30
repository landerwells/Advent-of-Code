use aochelpers;
use std::collections::HashMap;

type Coordinates = (i32, i32);

pub fn run() {
    let input: String = aochelpers::get_daily_input(3, 2015).unwrap();
    println!("Day Three Answers:");
    println!("Part One: {}", solve_part_one(input.clone()));
    println!("Part Two: {}", solve_part_two(input));
}

fn solve_part_one(input: String) -> i32 {
    let mut map: HashMap<Coordinates, i32> = HashMap::new();

    let mut current: Coordinates = (0, 0);
    let mut current2: Coordinates = (0, 0);

    map.insert(current, 1);
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => {
                    current = (current.0, current.1 + 1);
                    map.insert(current, 1);
                }
                'v' => {
                    current = (current.0, current.1 - 1);
                    map.insert(current, 1);
                }
                '>' => {
                    current = (current.0 + 1, current.1);
                    map.insert(current, 1);
                }
                '<' => {
                    current = (current.0 - 1, current.1);
                    map.insert(current, 1);
                }
                _ => {
                    panic!("what")
                }
            }
        } else {
            match c {
                '^' => {
                    current2 = (current2.0, current2.1 + 1);
                    map.insert(current2, 1);
                }
                'v' => {
                    current2 = (current2.0, current2.1 - 1);
                    map.insert(current2, 1);
                }
                '>' => {
                    current2 = (current2.0 + 1, current2.1);
                    map.insert(current2, 1);
                }
                '<' => {
                    current2 = (current2.0 - 1, current2.1);
                    map.insert(current2, 1);
                }
                _ => {
                    panic!("what")
                }
            }
        }
    }

    map.len() as i32
}

fn solve_part_two(input: String) -> i32 {
    0
}
