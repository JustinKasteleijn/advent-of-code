use std::collections::BTreeMap;
use std::ops::Not;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

use crate::problem::Problem;

#[derive(Debug)]
pub struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
pub struct Game<'a> {
    id: u32,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a> {
    fn valid(&self, map: &BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .any(|round| {
                round.iter().any(|cube| {
                    cube.amount > *map.get(cube.color).expect("This is valid")
                })
            })
            .not()
            .then_some(self.id)
    }

    fn power(&self) -> u32 {
        let mut map = BTreeMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        for round in self.rounds.iter() {
            for cube in round.iter() {
                let _ = map.entry(cube.color).and_modify(|v| *v = (*v).max(cube.amount)).or_insert(cube.amount);
            }
        }
        map.values().product()
    }
}

pub struct DayTwo;

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) =
        separated_pair(complete::u32, tag(" "), alpha1)(
            input
        )?;
    Ok((input, Cube { color, amount }))
}

fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) =
        separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        preceded(tag("Game "), complete::u32)(input)?;
    let (input, rounds) = preceded(
        tag(": "),
        separated_list1(tag("; "), round),
    )(input)?;
    Ok((input, Game { id, rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let map = BTreeMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);
        let (_, games) = parse_games(input).expect("This should not fail");
        games
            .into_iter()
            .filter_map(|game| game.valid(&map))
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_, games) = parse_games(input).expect("This should not fail");
        games
            .into_iter()
            .map(|game| game.power())
            .sum::<u32>()
            .to_string()
    }
}

mod tests {
    use crate::days::day02::day_two::DayTwo;
    use crate::problem::Problem;

    #[test]
    fn _part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(DayTwo.part_one(input), "8")
    }

    #[test]
    fn _part_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(DayTwo.part_two(input), "2286")
    }
}