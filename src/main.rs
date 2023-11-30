mod problem;
mod days;

use problem::Problem;
use days::*;

enum Days {
    Day1
}

fn day_to_problem(day: Days) -> Option<Box<dyn Problem>> {
    match day {
        Days::Day1 => Some(Box::new(day01::day_one::DayOne{}))
    }
}

fn main() {
    let problem = day_to_problem(Days::Day1);
    match problem {
        Some(problem) => println!("{}", problem.part_one(include_str!("../src/days/day01/input.txt"))),
        None => println!("Not yet implemented")
    }
}
