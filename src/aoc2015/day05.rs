use aochelpers;
use indoc::indoc;

pub fn run() {
    // let test_input = indoc!("jchzalrnumimnmhp");
    let test_input = indoc!("qjhvhtzxzqqjkmpb");
    let input: String = aochelpers::get_daily_input(5, 2015).unwrap();

    println!("Test: {}", is_nice(&test_input.to_string()));
    println!("Part One: {}", part_one(&input));
    // Part two was infinitely easier to implement using grep and regex,
    // since rust's regex crate does not have backreferencing.
    // Currently looking into rust crates that do though.
    // println!("---");
    // println!("Test: {}", part_two(&test_input.to_string()));
    // println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &String) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut count = 0;
    for line in &lines {
        if is_nice(line) {
            count += 1;
        }
    }
    count
}

fn is_nice(line: &String) -> bool {
    if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
        return false;
    }

    if line
        .chars()
        .collect::<Vec<_>>() // Collect into a vector to use `windows`
        .windows(2)
        .any(|window| window[0] == window[1])
        && line
            .chars()
            .filter(|&c| "aeiou".contains(c)) // Simplified vowel check
            .count()
            >= 3
    {
        return true;
    }
    false
}
