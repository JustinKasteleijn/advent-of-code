use nom::bytes::streaming::is_not;
use nom::character::complete;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::complete;
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use nom::sequence::separated_pair;
use nom_supreme::{ParserExt};
use crate::problem::Problem;

pub(crate) struct DaySix;

fn parse_numbers_p1(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list1(space1, complete::u32))
        .parse(input)
}

fn parse_p1(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(parse_numbers_p1, line_ending, parse_numbers_p1).parse(input)
}

fn parse_numbers_p2(input: &str) -> IResult<&str, u64> {
    is_not("0123456789")
        .precedes(separated_list1(space1, digit1)).map(|list|{
            list.join("").parse::<u64>().expect(":)")
        },
    )
        .parse(input)
}

fn parse_p2(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_numbers_p2, line_ending, parse_numbers_p2).parse(input)
}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let (_, (times, distances)) = parse_p1(input).expect("Must parse");

        let result = times
            .iter()
            .zip(distances)
            .map(|(time, record_distance)| {
                (0..*time)
                    .filter_map(|speed| {
                        let distance = (time - speed) * speed;
                        (distance > record_distance).then_some(distance)
                    }).count()
            }).product::<usize>();

        result.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_, (time, curr_record)) = parse_p2(input).expect("Must parse");

        let result =
                (0..time)
                    .filter_map(|speed| {
                        let distance = (time - speed) * speed;
                        (distance > curr_record).then_some(distance)
                    }).count();

        result.to_string()
    }
}

mod tests {
    use super::*;
    use crate::problem::Problem;

    #[test]
    fn part_one() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(DaySix.part_one(input), "288")
    }

    #[test]
    fn part_two() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(DaySix.part_two(input), "71503")
    }
}