use aochelpers;

pub fn run() {
    let input = aochelpers::get_daily_input(1, 2015).unwrap();

    let count = 0;
    for c in input {
      if c == '(' {
        count += 1;
      } else {
        count -= 1;
      }
    }

    println!("Day One Answers:");
    println!("{}", count);
}