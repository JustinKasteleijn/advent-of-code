use std::collections::{HashMap, HashSet};
use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::all_consuming;
use nom::{IResult, Parser};
use nom::multi::many1;
use nom::sequence::terminated;
use nom_locate::LocatedSpan;
use crate::problem::Problem;

pub(crate) struct DayTen;

#[derive(Debug, Eq, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    StartingPosition,
    Ground,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    In,
    Out,
}

#[derive(Debug)]
struct PipeInfo<'a> {
    span: SpanIVec2<'a>,
    pipe_type: PipeType,
}

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn with_xy(span: Span) -> SpanIVec2 {
    // column/location are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse(
    input: Span,
) -> IResult<Span, HashMap<IVec2, PipeType>> {
    let (input, output) =
        all_consuming(many1(terminated(
            alt((
                tag("|").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::Vertical,
                    }
                }),
                tag("-").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::Horizontal,
                    }
                }),
                tag("L").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::NorthEast,
                    }
                }),
                tag("J").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::NorthWest,
                    }
                }),
                tag("7").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::SouthWest,
                    }
                }),
                tag("F").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::SouthEast,
                    }
                }),
                tag("S").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type:
                        PipeType::StartingPosition,
                    }
                }),
                tag(".").map(with_xy).map(|span| {
                    PipeInfo {
                        span,
                        pipe_type: PipeType::Ground,
                    }
                }),
            )),
            multispace0,
        )))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .filter_map(|pipe_info| {
                Some((
                    pipe_info.span.extra,
                    pipe_info.pipe_type,
                ))
            })
            .collect(),
    ))
}

impl Problem for DayTen {
    fn part_one(&self, input: &str,
    ) -> String {
        let (_, grid) = parse(Span::new(input)).expect("Should parse");
        let start = grid
            .iter()
            .find_map(|(key, value)| {
                (value == &PipeType::StartingPosition)
                    .then_some(key)
            }).expect("Starting pos must exist");

        let north = *start + IVec2::new(0, -1);
        let north_position = grid
            .get(&north)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Vertical
                | PipeType::SouthWest
                | PipeType::SouthEast => true,
                _ => false,
            })
            .then_some((Direction::South, north));
        let south = *start + IVec2::new(0, 1);
        let south_position = grid
            .get(&south)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Vertical
                | PipeType::NorthWest
                | PipeType::NorthEast => true,
                _ => false,
            })
            .then_some((Direction::North, south));
        let east = *start + IVec2::new(1, 0);
        let east_position = grid
            .get(&east)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Horizontal
                | PipeType::NorthWest
                | PipeType::SouthWest => true,
                _ => false,
            })
            .then_some((Direction::West, east));
        let west = *start + IVec2::new(-1, 0);
        let west_position = grid
            .get(&west)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Horizontal
                | PipeType::NorthEast
                | PipeType::SouthEast => true,
                _ => false,
            })
            .then_some((Direction::East, west));

        let mut iters = vec![
            north_position,
            south_position,
            east_position,
            west_position,
        ]
            .into_iter()
            .flatten()
            .map(|tuple| {
                std::iter::successors(
                    Some(tuple),
                    |(from_direction, current_position)| {
                        let pipe_type = grid
                            .get(current_position)
                            .expect("should not be asking for a grid position that doesn't exist");

                        let direction_to_go = match (from_direction, pipe_type) {
                            (Direction::North, PipeType::Vertical) => Direction::South,
                            (Direction::North, PipeType::NorthEast) => Direction::East,
                            (Direction::North, PipeType::NorthWest) => Direction::West,
                            (Direction::South, PipeType::Vertical) => Direction::North,
                            (Direction::South, PipeType::SouthEast) => Direction::East,
                            (Direction::South, PipeType::SouthWest) => Direction::West,
                            (Direction::East, PipeType::Horizontal) => Direction::West,
                            (Direction::East, PipeType::NorthEast) => Direction::North,
                            (Direction::East, PipeType::SouthEast) => Direction::South,
                            (Direction::West, PipeType::Horizontal) => Direction::East,
                            (Direction::West, PipeType::NorthWest) => Direction::North,
                            (Direction::West, PipeType::SouthWest) => Direction::South,
                            value => { unreachable!("should not land on Ground or loop around again, {:?}", value); }
                        };
                        Some(match direction_to_go {
                            Direction::North => (Direction::South, *current_position + IVec2::new(0, -1)),
                            Direction::South => (Direction::North, *current_position + IVec2::new(0, 1)),
                            Direction::East => (Direction::West, *current_position + IVec2::new(1, 0)),
                            Direction::West => (Direction::East, *current_position + IVec2::new(-1, 0)),
                        })
                    },
                )
            });

        let path_a = iters.next().expect("path a should_exist");
        let path_b = iters.next().expect("path b should exist");
        let final_position = path_a
            .zip(path_b)
            .position(|(a, b)| a.1 == b.1)
            .expect("should meet in the middle");

        (final_position + 1).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_input, grid) = parse(Span::new(input))
            .expect("should parse a valid grid");

        let start_position = grid
            .iter()
            .find_map(|(key, value)| {
                (value == &PipeType::StartingPosition)
                    .then_some(key)
            })
            .expect("should have a starting position");

        let north = *start_position + IVec2::new(0, -1);
        let north_position = grid
            .get(&north)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Vertical
                | PipeType::SouthWest
                | PipeType::SouthEast => true,
                _ => false,
            })
            .then_some((Direction::South, north));
        let south = *start_position + IVec2::new(0, 1);
        let south_position = grid
            .get(&south)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Vertical
                | PipeType::NorthWest
                | PipeType::NorthEast => true,
                _ => false,
            })
            .then_some((Direction::North, south));
        let east = *start_position + IVec2::new(1, 0);
        let east_position = grid
            .get(&east)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Horizontal
                | PipeType::NorthWest
                | PipeType::SouthWest => true,
                _ => false,
            })
            .then_some((Direction::West, east));
        let west = *start_position + IVec2::new(-1, 0);
        let west_position = grid
            .get(&west)
            .is_some_and(|pipe_type| match pipe_type {
                PipeType::Horizontal
                | PipeType::NorthEast
                | PipeType::SouthEast => true,
                _ => false,
            })
            .then_some((Direction::East, west));

        let mut iters = vec![
            north_position,
            south_position,
            east_position,
            west_position,
        ]
            .into_iter()
            .flatten()
            .map(|tuple| {
                std::iter::successors(
                    Some(tuple),
                    |(from_direction, current_position)| {
                        let pipe_type = grid
                            .get(current_position)
                            .expect("should not be asking for a grid position that doesn't exist");

                        let direction_to_go = match (from_direction, pipe_type) {
                            (Direction::North, PipeType::Vertical) => Direction::South,
                            (Direction::North, PipeType::NorthEast) => Direction::East,
                            (Direction::North, PipeType::NorthWest) => Direction::West,
                            (Direction::South, PipeType::Vertical) => Direction::North,
                            (Direction::South, PipeType::SouthEast) => Direction::East,
                            (Direction::South, PipeType::SouthWest) => Direction::West,
                            (Direction::East, PipeType::Horizontal) => Direction::West,
                            (Direction::East, PipeType::NorthEast) => Direction::North,
                            (Direction::East, PipeType::SouthEast) => Direction::South,
                            (Direction::West, PipeType::Horizontal) => Direction::East,
                            (Direction::West, PipeType::NorthWest) => Direction::North,
                            (Direction::West, PipeType::SouthWest) => Direction::South,
                            value => {unreachable!("should not land on Ground or loop around again, {:?}", value);}
                        };
                        Some(match direction_to_go {
                            Direction::North => (Direction::South, *current_position + IVec2::new(0, -1)),
                            Direction::South => (Direction::North, *current_position + IVec2::new(0, 1)),
                            Direction::East => (Direction::West, *current_position + IVec2::new(1, 0)),
                            Direction::West => (Direction::East, *current_position + IVec2::new(-1, 0)),
                        })
                    },
                )
            });

        let path_a = iters.next().expect("path a should_exist");
        let path_b = iters.next().expect("path b should exist");
        let mut zip_it = path_a.zip(path_b);
        let mut pipe_locations: HashSet<IVec2> =
            HashSet::from([*start_position]);
        while let Some((path_a_node, path_b_node)) =
            zip_it.next()
        {
            pipe_locations.insert(path_a_node.1);
            pipe_locations.insert(path_b_node.1);

            if path_a_node.1 == path_b_node.1 {
                break;
            }
        }

        let result = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let mut status = Status::Out;

                line.chars()
                    .enumerate()
                    .filter(|(x, _)| {
                        let position =
                            IVec2::new(*x as i32, y as i32);
                        let pipe_type = grid
                            .get(&position)
                            .expect("should be a valid tile");
                        if pipe_locations.contains(&position) {
                            if [
                                PipeType::StartingPosition,
                                PipeType::Vertical,
                                PipeType::SouthWest,
                                PipeType::SouthEast,
                            ]
                                .contains(pipe_type)
                            {
                                status = match status {
                                    Status::In => Status::Out,
                                    Status::Out => Status::In,
                                };
                            };
                            false
                        } else {
                            match status {
                                Status::In => true,
                                Status::Out => false,
                            }
                        }
                    })
                    .count()
            })
            .sum::<usize>();

        result.to_string()
    }
}

// fn pip_line(input: &str) -> i32 {
//     let mut score = 0;
//     let mut crossings = 0;
//     for c in input.chars() {
//         match c {
//             '.' => {
//                 if crossings % 2 != 0 {
//                     score += 1;
//                 }
//             }
//             'S' => {
//                 // maybe
//                 crossings += 1;
//             }
//             '|' => {
//                 crossings += 1;
//             }
//             'F' => {
//                 crossings += 1;
//             }
//             '7' => {
//                 crossings += 1;
//             }
//             'L' => {}
//             'J' => {}
//             '-' => {}
//             value => unreachable!(
//                 "`{value}` is not a valid character"
//             ),
//         }
//     }
//     score
// }


#[cfg(test)]
mod tests {
    use crate::days::day10::day_ten::DayTen;
    use crate::problem::Problem;

    #[test]
    fn part_one() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!("8", DayTen.part_one(input));
    }

    #[test]
    fn _part_two() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(DayTen.part_two(input), "8");
    }
}
