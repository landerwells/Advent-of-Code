use aochelpers;
use indoc::indoc;

pub fn run() {
    let test_input = indoc!("");
    let input: String = aochelpers::get_daily_input(12, 2023).unwrap();

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

fn is_valid(groups: Vec<&str>, counts: Vec<i32>) -> bool {
    groups.len() == counts.len()
        && groups
            .iter()
            .zip(counts.iter())
            .all(|(g, &c)| g.len() as i32 == c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid(vec!["#", "##", "###"], vec![1, 2, 3]));
        assert!(is_valid(vec!["#", "###", "#"], vec![1, 3, 1]));
    }
}
