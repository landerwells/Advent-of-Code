use aochelpers;
use indoc::indoc;

pub fn run() {
    let test_input = indoc!("");
    let input: String = aochelpers::get_daily_input(10, 2022).unwrap();

    println!("Test: {}", part_one(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    0
}

fn part_two(input: &String) -> i32 {
    0
}
