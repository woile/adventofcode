use std::fs;

use itertools::Itertools;
use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::{delimited, preceded, separated, terminated},
    PResult, Parser,
};

type Seeds = Vec<u64>;
type TripletRaw = Vec<u64>;
// type SeedToSoil = HashMap<u64, u64>;
type Triplets = Vec<Triplet>;
type DestinationRange = u64;
type SourceRange = u64;
type Soil = Vec<Triplet>;
type Fertilizer = Vec<Triplet>;
type Water = Vec<Triplet>;
type Light = Vec<Triplet>;
type Temp = Vec<Triplet>;
type Humidity = Vec<Triplet>;
type Location = Vec<Triplet>;
type Range = (u64, u64);

#[derive(Debug, PartialEq)]
struct Triplet {
    destination_range: DestinationRange,
    source_range: SourceRange,
    length: u64,
}

impl Triplet {
    fn new(destination_range: DestinationRange, source_range: SourceRange, range: u64) -> Self {
        Self {
            destination_range,
            source_range,
            length: range,
        }
    }

    fn as_source_range(&self) -> (u64, u64) {
        (self.source_range, self.source_range + self.length)
    }

    fn as_destination_range(&self) -> (u64, u64) {
        (self.destination_range, self.destination_range + self.length)
    }
}

#[derive(Debug, PartialEq)]
struct SeedRange {
    start: u64,
    length: u64,
}

impl SeedRange {
    fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }
    fn as_range(&self) -> Range {
        (self.start, self.start + self.length)
    }
}

fn parse_seeds(input: &mut &str) -> PResult<Seeds> {
    preceded(
        "seeds: ",
        separated(0.., digit1.try_map(|v: &str| v.parse::<u64>()), multispace1),
    )
    .parse_next(input)
}

fn parse_seed_ranges(input: &mut &str) -> PResult<Vec<SeedRange>> {
    parse_seeds
        .map(|v| {
            v.iter()
                .tuples()
                .map(|(start, length)| (SeedRange::new(*start, *length)))
                .collect::<Vec<SeedRange>>()
        })
        .parse_next(input)
}

fn parse_triplet(input: &mut &str) -> PResult<Triplet> {
    separated(3, digit1.try_map(|v: &str| v.parse::<u64>()), " ")
        .map(|v: TripletRaw| {
            let mut iter = v.into_iter();
            Triplet::new(
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .parse_next(input)
}

fn parse_triplets(input: &mut &str) -> PResult<Triplets> {
    separated(0.., parse_triplet, "\n")
        .map(|mut v: Triplets| {
            v.sort_by(|a, b| a.destination_range.cmp(&b.destination_range));
            v
        })
        .parse_next(input)
}
fn parse_seeds_to_soil(input: &mut &str) -> PResult<Triplets> {
    preceded(
        delimited(multispace0, "seed-to-soil map:", multispace0),
        parse_triplets,
    )
    .parse_next(input)
}

fn parse_soil_to_fertilizer(input: &mut &str) -> PResult<Triplets> {
    preceded(
        delimited(multispace0, "soil-to-fertilizer map:", multispace0),
        parse_triplets,
    )
    .parse_next(input)
}

fn parse_fertilizer_to_water(input: &mut &str) -> PResult<Triplets> {
    preceded(
        delimited(multispace0, "fertilizer-to-water map:", multispace0),
        parse_triplets,
    )
    .parse_next(input)
}

fn parse_water_to_light(input: &mut &str) -> PResult<Triplets> {
    preceded(
        delimited(multispace0, "water-to-light map:", multispace0),
        parse_triplets,
    )
    .parse_next(input)
}

fn parse_light_to_temperature(input: &mut &str) -> PResult<Triplets> {
    preceded(
        delimited(multispace0, "light-to-temperature map:", multispace0),
        parse_triplets,
    )
    .parse_next(input)
}

fn parse_temperature_to_humidity(input: &mut &str) -> PResult<Triplets> {
    preceded(
        delimited(multispace0, "temperature-to-humidity map:", multispace0),
        parse_triplets,
    )
    .parse_next(input)
}

//
fn parse_humidity_to_location(input: &mut &str) -> PResult<Triplets> {
    terminated(
        preceded(
            delimited(multispace0, "humidity-to-location map:", multispace0),
            parse_triplets,
        ),
        multispace0,
    )
    .parse_next(input)
}

fn parse_map(
    input: &mut &str,
) -> PResult<(
    Seeds,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location,
)> {
    (
        parse_seeds,
        parse_seeds_to_soil,
        parse_soil_to_fertilizer,
        parse_fertilizer_to_water,
        parse_water_to_light,
        parse_light_to_temperature,
        parse_temperature_to_humidity,
        parse_humidity_to_location,
    )
        .parse_next(input)
}

fn source_dest_map(val: u64, triplet: &Triplet) -> Option<u64> {
    let (start, end) = triplet.as_source_range();
    if val >= start && val < end {
        let dest = triplet.as_destination_range();
        let r = val - start;
        return Some(r + dest.0);
    }
    None
}

fn parse_part1(input: &mut &str) -> PResult<u64> {
    let (seeds, soil, fertilizer, water, light, temp, humidity, locations) =
        parse_map.parse_next(input).expect("Failed to parse map");

    let loc = seeds
        .into_iter()
        .map(|seed| {
            let r = soil
                .iter()
                .filter_map(move |triplet| source_dest_map(seed, triplet))
                .collect_vec();
            if r.len() == 0 {
                return vec![seed];
            }
            return r;
        })
        .flatten()
        .map(|seed| {
            let r = fertilizer
                .iter()
                .filter_map(move |triplet| source_dest_map(seed, triplet))
                .collect_vec();
            if r.len() == 0 {
                return vec![seed];
            }
            return r;
        })
        .flatten()
        .map(|seed| {
            let r = water
                .iter()
                .filter_map(move |triplet| source_dest_map(seed, triplet))
                .collect_vec();
            if r.len() == 0 {
                return vec![seed];
            }
            return r;
        })
        .flatten()
        .map(|seed| {
            let r = light
                .iter()
                .filter_map(move |triplet| source_dest_map(seed, triplet))
                .collect_vec();
            if r.len() == 0 {
                return vec![seed];
            }
            return r;
        })
        .flatten()
        .map(|seed| {
            let r = temp
                .iter()
                .filter_map(move |triplet| source_dest_map(seed, triplet))
                .collect_vec();
            if r.len() == 0 {
                return vec![seed];
            }
            return r;
        })
        .flatten()
        .map(|seed| {
            let r = humidity
                .iter()
                .filter_map(move |triplet| source_dest_map(seed, triplet))
                .collect_vec();
            if r.len() == 0 {
                return vec![seed];
            }
            return r;
        })
        .flatten()
        .map(|seed| {
            let r = locations
                .iter()
                .filter_map(move |triplet| source_dest_map(seed, triplet))
                .collect_vec();
            if r.len() == 0 {
                return vec![seed];
            }
            return r;
        })
        .flatten()
        .min();

    Ok(loc.unwrap())

}

fn main() {
    let result = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let out = parse_part1.parse(&result).unwrap();
    println!("{}", out);
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 1 2 3 4 5";
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(parse_seeds.parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_triplets() {
        let input = "1 2 3";
        let expected = Triplet::new(1, 2, 3);
        // let expected = vec![1, 2, 3];
        assert_eq!(parse_triplet.parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_seeds_to_soil() {
        let input = "seed-to-soil map:\n1 2 3";
        let expected = vec![Triplet::new(1, 2, 3)];
        assert_eq!(parse_seeds_to_soil.parse(input), Ok(expected));
    }

    #[test]
    fn test_parse_seed_ranges() {
        let input = "seeds: 1 2 3 2";
        let expected = vec![SeedRange::new(1, 2), SeedRange::new(3, 4)];
        assert_eq!(parse_seed_ranges.parse(input), Ok(expected));
    }

    #[test]
    fn test_source_dest_map() {
        let triplet = Triplet::new(37, 52, 2);
        let expected = Some(38);
        let out = source_dest_map(53, &triplet);
        assert_eq!(out, expected);
        let expected = None;
        let out = source_dest_map(54, &triplet);
        assert_eq!(out, expected);
    }
}
