use itertools::Itertools;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::{IResult, Parser};
use nom::multi::separated_list1;
use crate::problem::Problem;

pub(crate) struct DayNine;

fn part_one(top_row: Vec<i32>) -> i64 {
    let mut rows: Vec<Vec<i32>> = vec![top_row];
    while rows.last().unwrap().len() > 1 &&
        !rows.last().unwrap().iter().all(|x| *x == 0)
    {
        let row = rows.last().unwrap();
        rows.push(
            row
                .iter()
                .tuple_windows()
                .fold(Vec::new(), |mut acc, (x, y)| {
                    acc.push(*y - *x);
                    acc
                })
        )
    }
    rows
        .iter()
        .rev()
        .fold(0, |mut acc, row| {
            acc += *row.last().unwrap() as i64;
            acc
        })
}

fn part_two(top_row: Vec<i32>) -> i64 {
    let mut rows: Vec<Vec<i32>> = vec![top_row];
    while rows.last().unwrap().len() > 1 &&
        !rows.last().unwrap().iter().all(|x| *x == 0)
    {
        let row = rows.last().unwrap();
        rows.push(
            row
                .iter()
                .tuple_windows()
                .fold(Vec::new(), |mut acc, (x, y)| {
                    acc.push(*x - *y);
                    acc
                })
        )
    }
    rows
        .iter()
        .rev()
        .fold(0, |mut acc, row| {
            acc += *row.first().unwrap() as i64;
            acc
        })
}


fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(
        line_ending,
        separated_list1(space1, complete::i32))
        (input)
}

impl Problem for DayNine {
    fn part_one(&self, input: &str) -> String {
        let (_, lines) = parse(input).expect("Must parse");
        lines
            .into_iter()
            .map(|row| {
                part_one(row)
            }).sum::<i64>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_, lines) = parse(input).expect("Must parse");
        lines
            .into_iter()
            .map(|row| {
                part_two(row)
            }).sum::<i64>()
            .to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::days::day10::day_ten::DayTen;
    use super::*;

    #[test]
    fn _part_one() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(DayNine.part_one(input), "114")
    }

    #[test]
    fn _part_two() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(DayNine.part_two(input), "2")
    }
}
