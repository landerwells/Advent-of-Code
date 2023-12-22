use std::env;

pub fn run() {
    env::set_var("AOCTOKEN", "53616c7465645f5ff30847a61c609fca0373a9571a633ffb28d7209b03e95add495275dc91b67497d11eadc584912ffe03e716e3c719655e3acfc9542ae5a5f7");
    let input = aochelpers::get_daily_input(12, 2023).unwrap();

    println!("Day Twelve Answers:");
    println!("Part One: {}", solve_part_one(input.clone()));
    // println!("Part Two: {}", );
}

fn solve_part_one(input: String) -> i32 {
  let lines: Vec<String> = input.lines().map(String::from).collect();

  for line in lines {
    println!("{}", line);
  }

  0
}
