use std::env;

// mod 2022;
mod aoc2023;
// ... import other year modules

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run_year(&args[1]),
        3 => run_day(&args[1], &args[2]),
        _ => eprintln!("Usage: program [YEAR] or program [YEAR] [DAY]"),
    }
}

fn run_year(year: &str) {
    match year {
        // "2021" => year2021::run_all(),
        "2023" => aoc2023::run_all(),
        // ... handle other years
        _ => eprintln!("Invalid year"),
    }
}

fn run_day(year: &str, day: &str) {
    match (year, day) {
        ("2023", "01") => aoc2023::day01::run(),
        ("2023", "02") => aoc2023::day02::run(),
        ("2023", "03") => aoc2023::day03::run(),
        ("2023", "04") => aoc2023::day04::run(),
        ("2023", "05") => aoc2023::day05::run(),
        ("2023", "06") => aoc2023::day06::run(),
        ("2023", "07") => aoc2023::day07::run(),
        ("2023", "08") => aoc2023::day08::run(),
        ("2023", "09") => aoc2023::day09::run(),
        ("2023", "10") => aoc2023::day10::run(),
        ("2023", "11") => aoc2023::day11::run(),
        ("2023", "12") => aoc2023::day12::run(),
        // ... handle other years and days
        _ => eprintln!("Invalid year or day"),
    }
}

