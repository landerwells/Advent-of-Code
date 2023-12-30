use aochelpers;

pub fn run() {
    let input = aochelpers::get_daily_input(1, 2015).unwrap();
    println!("Day One Answers:");
    println!(
        "Part One: {}",
        input.chars().fold(0, |acc, c| {
            if c == '(' {
                acc + 1
            } else {
                acc - 1
            }
        })
    );
    println!("Part One: {}", solve_part_two(input));
}

fn solve_part_two(input: String) -> i32 {
    let mut count = 0;
    let mut index = 1;
    for c in input.chars() {
        if c == '(' {
            count += 1;
        } else {
            count -= 1;
        }
        if count == -1 {
            return index;
        }
        index += 1;
    }
    index
}
