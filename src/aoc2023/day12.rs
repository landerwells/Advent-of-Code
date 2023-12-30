use aochelpers;

pub fn run() {
    let input = aochelpers::get_daily_input(12, 2023).unwrap();

    println!("Day Twelve Answers:");
    println!("Part One: {}", solve_part_one(input.clone()));
    // println!("Part Two: {}", );
}

fn solve_part_one(input: String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();

    // for line in lines {
    //   println!("{}", line);
    // }

    0
}
