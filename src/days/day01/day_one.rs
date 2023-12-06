use crate::problem::Problem;

pub struct DayOne;

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let first_digit = input
            .lines()
            .flat_map(|s| s.
                chars()
                .find(|c| c.
                    is_digit(10))
            )
            .collect::<Vec<char>>();

        let last_digit = input
            .lines()
            .flat_map(|s| s.
                chars()
                .rev()
                .find(|c| c.
                    is_digit(10))
            )
            .collect::<Vec<char>>();

        let sum = first_digit
            .iter()
            .zip(last_digit.iter())
            .into_iter()
            .map(|(&x, &y)| {
                (x.to_string() + &y.to_string()).parse::<u32>().unwrap()
            })
            .sum::<u32>();
        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let mut it = (0..line.len()).filter_map(|index| {
                    let reduced_line = &line[index..];
                    let result = if reduced_line.starts_with("one") {
                        '1'
                    } else if reduced_line.starts_with("two") {
                        '2'
                    } else if reduced_line.starts_with("three") {
                        '3'
                    } else if reduced_line.starts_with("four") {
                        '4'
                    } else if reduced_line.starts_with("five") {
                        '5'
                    } else if reduced_line.starts_with("six") {
                        '6'
                    } else if reduced_line.starts_with("seven") {
                        '7'
                    } else if reduced_line.starts_with("eight") {
                        '8'
                    } else if reduced_line.starts_with("nine") {
                        '9'
                    } else {
                        reduced_line.chars().next().unwrap()
                    };
                    result.to_digit(10)
                });
                let first = it.next().expect("Must be a number");

                match it.last() {
                    Some(num) => format!("{first}{num}"),
                    None => format!("{first}{first}")
                }.parse::<u32>().expect("Must be a number")
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _part_one() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(DayOne.part_one(input), "142")
    }
    #[test]
    fn _part_two() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(DayOne.part_two(input), "281");
    }
}
