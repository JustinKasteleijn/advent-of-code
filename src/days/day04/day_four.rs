use std::collections::BTreeMap;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{space1};
use nom::{IResult};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use crate::problem::Problem;

pub struct DayFour;

#[derive(Debug, PartialEq, Clone)]
struct Card {
    number: u32,
}

#[derive(Debug)]
struct Game {
    cards_in_hand: Vec<Card>,
    cards_in_deck: Vec<Card>,
    id: u32,
}

impl Game {
    fn get_score(&self) -> u32 {
        let score = self
            .cards_in_hand
            .iter()
            .filter_map(|x| {
                match self.cards_in_deck.contains(x) {
                    true => Some(1),
                    false => None
                }
            })
            .count() as u32;
        match score {
            s if s >= 1 => 2u32.pow(s - 1),
            _ => 0
        }
    }

    fn get_matches(&self) -> u32 {
        self
            .cards_in_hand
            .iter()
            .filter_map(|x| {
                match self.cards_in_deck.contains(x) {
                    true => Some(1),
                    false => None
                }
            })
            .count() as u32
    }
}

fn parse_cards(line: &str) -> IResult<&str, Vec<u32>> {
    let (line, cards) = preceded(space1, separated_list1(space1, complete::u32))(line)?;
    Ok((line, cards))
}

fn parse_game(line: &str) -> IResult<&str, Game> {
    let (line, id) = terminated(preceded(tuple((tag("Card"), space1)), complete::u32), tag(":"))(line)?;
    let (line, (cards_in_hand, cards_in_deck)) = separated_pair(
        parse_cards,
        tuple((space1, tag("|"))),
        parse_cards,
    )(line)?;
    let cards_in_hand = cards_in_hand.iter().map(|card| Card { number: *card }).collect::<Vec<Card>>();
    let cards_in_deck = cards_in_deck.iter().map(|card| Card { number: *card }).collect::<Vec<Card>>();
    Ok((line, Game { cards_in_hand, cards_in_deck, id }))
}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let games = input
            .lines()
            .map(|line|
                parse_game(line).expect("Must parse"))
            .collect::<Vec<(&str, Game)>>();

        let x = games
            .iter()
            .map(|(_, game)| game.get_score())
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();
        x.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let games = input
            .lines()
            .map(|line|
                parse_game(line).expect("Must parse"))
            .collect::<Vec<(&str, Game)>>();

        let data = games
            .iter()
            .map(|(_, game)| game.get_matches())
            .collect::<Vec<u32>>();

        let store = games
            .iter()
            .map(|(_, game)| ((game.id  - 1) as usize, 1))
            .collect::<BTreeMap<usize, u32>>();

        let result = data
            .iter()
            .enumerate()
            .fold(store, |mut acc, (index, card_score)| {
                let to_add = *acc.get(&index).unwrap();

                for i in (index + 1)..(index + 1 + *card_score as usize)
                {
                    acc.entry(i).and_modify(|value| {
                        *value += to_add;
                    });
                }
                acc
            })
            .values()
            .sum::<u32>();

        result.to_string()
    }
}

mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(DayFour.part_one(input), "13")
    }

    #[test]
    fn part_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(DayFour.part_two(input), "30")
    }
}