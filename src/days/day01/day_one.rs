use crate::problem::Problem;

pub struct DayOne;

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        format!("{}", "Day 1, Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Day 1, Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _part_one() {
        assert_eq!(DayOne.part_one(), );
    }
}
