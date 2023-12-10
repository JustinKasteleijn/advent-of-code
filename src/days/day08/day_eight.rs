use std::collections::BTreeMap;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::{multispace1, space1};
use nom::combinator::opt;
use nom::error::Error;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair};

use crate::problem::Problem;

pub struct DayEight;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

fn parse_source(input: &str) -> IResult<&str, &str> {
    take_until(" ")(input)
}

fn parse_nodes(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        preceded(tag("("), take_until(",")),
        tag(","),
        preceded(space1, take_until(")")),
    )(input)
}

fn parse_tree(input: &str) -> IResult<&str, (&str, (&str, &str)), Error<&str>> {
    let (input, _) = opt(multispace1)(input)?;

    separated_pair(parse_source, tag(" = "), parse_nodes)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt(
        (complete::char('R').map(|_| Direction::RIGHT), complete::char('L').map(|_| Direction::LEFT))
    ))(input)
}

impl Problem for DayEight {
    fn part_one(&self, input: &str) -> String {
        let (input, instructions) = parse_instruction(input).expect(":)");

        let tree: BTreeMap<&str, (&str, &str)> = input
            .lines()
            .fold(BTreeMap::new(), |mut acc, line| {
                if let Ok((_, (src, (left, right)))) = parse_tree(line) {
                    let value = (left, right);
                    acc.insert(src, value);
                }
                acc
            });

        let mut current_node = "AAA";
        const DESTINATION: &str = "ZZZ";
        let Some(step_count) = instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(i, direction)| {
                let nodes = tree
                    .get(current_node)
                    .expect("Must be a valid node");

                let next_node = match direction {
                    Direction::LEFT => nodes.0,
                    Direction::RIGHT => nodes.1,
                };

                if next_node == DESTINATION {
                    Some(i + 1)
                } else {
                    current_node = next_node;
                    None
                }
            })
        else {
            panic!("Infinite iterator")
        };

        step_count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (input, instructions) = parse_instruction(input).expect(":)");

        let tree: BTreeMap<&str, (&str, &str)> = input
            .lines()
            .fold(BTreeMap::new(), |mut acc, line| {
                if let Ok((_, (src, (left, right)))) = parse_tree(line) {
                    let value = (left, right);
                    acc.insert(src, value);
                }
                acc
            });

        let current_nodes: Vec<&str> = tree
            .keys()
            .filter(|key| key.ends_with("A"))
            .cloned()
            .collect();

        let results = current_nodes
            .iter()
            .map(|node| {
                let mut current_node = *node;

                instructions
                    .iter()
                    .cycle()
                    .enumerate()
                    .find_map(|(i, direction)| {
                        let nodes = tree
                            .get(current_node)
                            .expect("Must be a valid node");

                        let next_node = match direction {
                            Direction::LEFT => nodes.0,
                            Direction::RIGHT => nodes.1,
                        };

                        if next_node.ends_with('Z') {
                            Some(i + 1)
                        } else {
                            current_node = next_node;
                            None
                        }
                    }).expect(":)")
            })
            .collect::<Vec<usize>>();

        lcm(&results).to_string()
    }
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0]
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use crate::days::day08::day_eight::DayEight;
    use crate::problem::Problem;

    #[test]
    fn _part_one() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(DayEight.part_one(input), "6")
    }

    #[test]
    fn _part_two() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(DayEight.part_two(input), "6")
    }
}