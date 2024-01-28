use aochelpers;
use indoc::indoc;

pub fn run() {
    let test_input1 = indoc!(
        "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"
    );
    let test_input2 = indoc!(
        "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"
    );
    let input = aochelpers::get_daily_input(1, 2023).unwrap();

    println!("Test: {}", part_one(&test_input1.to_string()));
    println!("Part One: {}", part_one(&input));
    println!("---");
    println!("Test: {}", part_two(&test_input2.to_string()));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let mut sum: i32 = 0;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    for line in lines {
        let numbers: String = line.chars().filter(|&c| c.is_numeric()).collect();
        let first = numbers.chars().next().unwrap();
        let last = numbers.chars().last().unwrap();
        let result = format!("{}{}", first, last);
        let result_int: i32 = result.parse().unwrap();
        sum += result_int;
    }
    sum
}

pub fn part_two(input: &String) -> u32 {
    input
        .split("\n")
        .map(|line| {
            line.to_string()
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| {
                    c.to_digit(10)
                        .expect("Failed to convert character to digit")
                })
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            10 * vec.first().expect("Every line must have atleast one digit")
                + vec.last().expect("Every line must have atleast one digit")
        })
        .sum()
}
