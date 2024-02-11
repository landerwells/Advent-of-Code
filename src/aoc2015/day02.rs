use aochelpers;

pub fn run() {
    let input: String = aochelpers::get_daily_input(2, 2015).unwrap();
    println!("Day Two Answers:");
    println!("Part One: {}", solve_part_one(&input));
    println!("Part Two: {}", solve_part_two(&input));
}

fn solve_part_one(input: &str) -> i32 {
    let dimension_list: Vec<String> = input.lines().map(String::from).collect();

    let mut sum = 0;
    for dimension in dimension_list {
        sum += calculate_wrapping_paper(dimension);
    }
    sum
}

fn solve_part_two(input: &str) -> i32 {
    let dimension_list: Vec<String> = input.lines().map(String::from).collect();

    let mut sum = 0;
    for dimension in dimension_list {
        sum += calculate_ribbon(dimension);
    }
    sum
}

fn calculate_ribbon(dimensions: String) -> i32 {
    let mut dim_array: Vec<i32> = dimensions
        .split("x")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let l = dim_array[0];
    let w = dim_array[1];
    let h = dim_array[2];
    let volume = l * w * h;

    if let Some(max_index) = dim_array
        .iter()
        .enumerate()
        .max_by_key(|&(_, &val)| val)
        .map(|(index, _)| index)
    {
        dim_array.remove(max_index);
    }
    let perimeter = dim_array.iter().map(|x| x * 2).sum::<i32>();

    volume + perimeter
}

fn calculate_wrapping_paper(dimensions: String) -> i32 {
    let dim_array: Vec<&str> = dimensions.split("x").collect();
    let l = dim_array[0].parse::<i32>().unwrap();
    let w = dim_array[1].parse::<i32>().unwrap();
    let h = dim_array[2].parse::<i32>().unwrap();
    let sides = vec![l * w, w * h, h * l];

    2 * (sides.iter().sum::<i32>()) + sides.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_wrapping_paper() {
        let mut input = "2x3x4".to_string();
        assert_eq!(58, calculate_wrapping_paper(input));
        input = "1x1x10".to_string();
        assert_eq!(43, calculate_wrapping_paper(input));
        input = "20x3x11".to_string();
        assert_eq!(659, calculate_wrapping_paper(input));
    }

    #[test]
    fn test_calculate_ribbon() {
        let mut input = "2x3x4".to_string();
        assert_eq!(34, calculate_ribbon(input));
        input = "1x1x10".to_string();
        assert_eq!(14, calculate_ribbon(input));
        input = "20x3x11".to_string();
        assert_eq!(688, calculate_ribbon(input));
    }
}
