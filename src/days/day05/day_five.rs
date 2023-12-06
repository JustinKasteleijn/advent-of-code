use std::ops::Range;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom_supreme::{ParserExt, tag::complete::tag};
use nom::{IResult, Parser};
use nom::bytes::complete::take_until;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, tuple};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::problem::Problem;

pub(crate) struct DayFive;

// struct SeedId(u32)

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, src: u64) -> u64 {
        let valid = self.mappings.iter().find(|(src_range, _)| {
            src_range.contains(&src)
        });

        let Some((src_range, dst_range)) = valid else {
            return src;
        };

        let offset= src - src_range.start;

        dst_range.start + offset
    }
}

fn line_p1(
    input: &str,
) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (
            source..(source + num),
            destination..(destination + num),
        ),
    ))
}

fn seed_map_p1(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(
            many1(line_ending.precedes(line_p1))
                .map(|mappings| SeedMap { mappings }),
        )
        .parse(input)
}

fn parse_seedmaps_p1(
    input: &str,
) -> IResult<&str, (Vec<u64>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;
    let (input, maps) = many1(seed_map_p1)(input)?;

    Ok((input, (seeds, maps)))
}

fn line_p2(
    input: &str,
) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input)?;

    Ok((
        input,
        (
            source..(source + num),
            destination..(destination + num),
        ),
    ))
}

fn seed_map_p2(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(
            many1(line_ending.precedes(line_p2))
                .map(|mappings| SeedMap { mappings }),
        )
        .parse(input)
}

fn parse_seedmaps_p2(
    input: &str,
) -> IResult<&str, (Vec<Range<u64>>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, tag(" "), complete::u64).map(|(start, offset)| {
                start..(start+offset)
            })))
        .parse(input)?;
    let (input, maps) = many1(seed_map_p2)(input)?;

    Ok((input, (seeds, maps)))
}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let (_, (seeds, maps)) = parse_seedmaps_p1(input).expect(":)");

        let locations = seeds
            .iter()
            .map(|seed| {
                maps.iter().fold(*seed, |seed, map| {
                    map.translate(seed)
                })
            }).collect::<Vec<u64>>();

        locations.iter().min().expect("Should have minimum").to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_, (seeds, maps)) = parse_seedmaps_p2(input).expect(":)");

        let minimum_location = seeds
            .into_par_iter()
            // .progress_count(count)
            .flat_map(|range| range.clone())
            .map(|seed| {
                maps.iter()
                    .fold(seed, |seed, map| map.translate(seed))
            })
            .min();

        minimum_location
            .expect("should have a minimum location value")
            .to_string()
    }
}

mod tests {
    use crate::days::day05::day_five::DayFive;
    use crate::problem::Problem;

    #[test]
    fn part_one() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(DayFive.part_one(input), "35")
    }

    #[test]
    fn part_two() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(DayFive.part_two(input), "46")
    }
}