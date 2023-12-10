#![feature(is_some_and)]

mod problem;
mod days;

use problem::Problem;
use days::*;

enum Days {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
}

fn day_to_problem(day: Days) -> Option<Box<dyn Problem>> {
    match day {
        Days::Day1 => Some(Box::new(day01::day_one::DayOne{})),
        Days::Day2 => Some(Box::new(day02::day_two::DayTwo{})),
        Days::Day3 => Some(Box::new(day03::day_three::DayThree{})),
        Days::Day4 => Some(Box::new(day04::day_four::DayFour{})),
        Days::Day5 => Some(Box::new(day05::day_five::DayFive{})),
        Days::Day6 => Some(Box::new(day06::day_six::DaySix{})),
        Days::Day7 => Some(Box::new(day07::day_seven::DaySeven{})),
        Days::Day8 => Some(Box::new(day08::day_eight::DayEight{})),
        Days::Day9 => Some(Box::new(day09::day_nine::DayNine{})),
        Days::Day10 => Some(Box::new(day10::day_ten::DayTen{})),
    }
}

fn main() {
    let problem = day_to_problem(Days::Day9);
    match problem {
        Some(problem) => println!("{}", problem.part_two(include_str!("../src/days/day09/input.txt"))),
        None => println!("Not yet implemented")
    }
}
