use aochelpers;
use indoc::indoc;

pub fn run() {
    let test_input = indoc!(
        "1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000"
    );
    let input: String = aochelpers::get_daily_input(1, 2022).unwrap();

    println!("Test: {}", part_one(test_input.to_string()));
    println!("Part One: {}", part_one(input.clone()));
    println!("---");
    println!("Test: {}", part_two(test_input.to_string()));
    println!("Part Two: {}", part_two(input));
}

fn part_one(input: String) -> i32 {
    *calories(input).iter().max().unwrap()
}

fn part_two(input: String) -> i32 {
    let mut sums: Vec<i32> = calories(input);

    sums.sort_unstable_by(|a, b| b.cmp(a));

    sums.iter().take(3).sum()
}

fn calories(input: String) -> Vec<i32> {
    let elves: Vec<String> = input.split("\n\n").map(String::from).collect();

    elves
        .iter()
        .map(|elf| {
            elf.lines()
                .map(|line| line.to_string().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}
