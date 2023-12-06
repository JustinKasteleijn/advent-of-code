use std::collections::{BTreeMap, HashSet};
use crate::problem::Problem;

pub struct DayThree;

#[derive(Debug)]
enum Value {
    Empty,
    Number(u32),
    Symbol(char),
}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        //To grid
        let v = input.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().map(|(x, character)| {
                ((y as i32, x as i32), match character {
                    '.' => Value::Empty,
                    c if c.is_ascii_digit() => {
                        Value::Number(c.to_digit(10).expect("Should be a digit"))
                    }
                    c => Value::Symbol(c),
                })
            }).collect::<Vec<((i32, i32), Value)>>()
        }).collect::<BTreeMap<(i32, i32), Value>>();

        let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
        for ((y, x), value) in v.iter() {
            if let Value::Number(num) = value {
                match numbers.iter().last() {
                    Some(v) => {
                        let last = v.iter().last();
                        match last {
                            Some(((last_x, _), _)) => {
                                if last_x + 1 == *x {
                                    let l = numbers.iter_mut().last().expect("Should exist");
                                    l.push(((*x, *y), *num))
                                } else {
                                    numbers.push(vec![((*x, *y), *num)])
                                }
                            }
                            None => unimplemented!("Dont come here")
                        }
                    }
                    None => {
                        numbers.push(vec![((*x, *y), *num)])
                    }
                }
            }
        }
        let mut total = 0;
        for num_list in numbers {
            // (x,y)
            let positions = [
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];
            let num_positions: Vec<(i32, i32)> = num_list
                .iter()
                .map(|((y, x), _)| (*x as i32, *y as i32))
                .collect();
            let pos_to_check: HashSet<(i32, i32)> = num_list
                .iter()
                .flat_map(|(pos, _)| {
                    positions.iter().map(|outer_pos| {
                        // outer_pos.x + pos.x, .y + .y
                        (
                            outer_pos.0 + pos.1 as i32,
                            outer_pos.1 + pos.0 as i32,
                        )
                    })
                })
                .filter(|num| !num_positions.contains(num))
                .collect::<HashSet<(i32, i32)>>();

            let is_part_num: bool = pos_to_check.iter().any(|pos| {
                let val = v.get(&pos);
                if let Some(Value::Symbol(_)) = val {
                    true
                } else {
                    false
                }
            });

            if is_part_num {
                total += num_list
                    .iter()
                    .map(|(_, num)| num.to_string())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
            }
        }
        total.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        //To grid
        let v = input.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().map(|(x, character)| {
                ((y as i32, x as i32), match character {
                    '.' => Value::Empty,
                    c if c.is_ascii_digit() => {
                        Value::Number(c.to_digit(10).expect("Should be a digit"))
                    }
                    c => Value::Symbol(c),
                })
            }).collect::<Vec<((i32, i32), Value)>>()
        }).collect::<BTreeMap<(i32, i32), Value>>();

        let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
        for ((y, x), value) in v.iter() {
            if let Value::Number(num) = value {
                match numbers.iter().last() {
                    Some(v) => {
                        let last = v.iter().last();
                        match last {
                            Some(((last_x, _), _)) => {
                                if last_x + 1 == *x {
                                    let l = numbers.iter_mut().last().expect("Should exist");
                                    l.push(((*x, *y), *num))
                                } else {
                                    numbers.push(vec![((*x, *y), *num)])
                                }
                            }
                            None => unimplemented!("Dont come here")
                        }
                    }
                    None => {
                        numbers.push(vec![((*x, *y), *num)])
                    }
                }
            }
        }
        let mut total = 0;
        for symbol in v.iter().filter(|(_, value)| {
            match value {
                Value::Symbol('*') => true,
                _ => false
            }
        }) {
            // (x,y)
            let positions = [
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ];

            let pos_to_check: Vec<(i32, i32)> = positions
                .iter()
                .map(|adjacent| {
                    (
                        adjacent.0 + symbol.0 .1,
                        adjacent.1 + symbol.0 .0,
                    )
                })
                .collect();

            let mut numbers_to_symbols = HashSet::new();

            for pos in pos_to_check {
                for (i, num_list) in numbers.iter().enumerate() {
                    if num_list
                        .iter()
                        .find(|(num_pos, _)| num_pos == &pos)
                        .is_some()
                    {
                        numbers_to_symbols.insert(i);
                    }
                }
            }

            let is_gear = numbers_to_symbols.iter().count() == 2;

            if is_gear {
                total += numbers_to_symbols
                    .iter()
                    .map(|index| {
                        numbers[*index]
                            .iter()
                            .map(|(_, num)| num.to_string())
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap()
                    })
                    .product::<usize>()
            }
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _part_one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(DayThree.part_one(input), "4361")
    }
}