use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::{delimited, preceded, separated, separated_pair, terminated},
    token::tag,
    PResult, Parser,
};

type WinningNumbers = HashSet<u64>;
type TicketNumbers = HashSet<u64>;

fn parse_numbers<'s>(i: &mut &'s str) -> PResult<Vec<u64>> {
    separated(0.., digit1.try_map(|v: &str| v.parse::<u64>()), multispace1).parse_next(i)
}

fn parse_num_pairs<'s>(i: &mut &'s str) -> PResult<(WinningNumbers, TicketNumbers)> {
    separated_pair(
        parse_numbers.map(WinningNumbers::from_iter),
        delimited(multispace0, "|", multispace0),
        parse_numbers.map(TicketNumbers::from_iter),
    )
    .parse_next(i)
}

fn parse_card_id<'s>(i: &mut &'s str) -> PResult<usize> {
    preceded(
        tag("Card"),
        preceded(multispace0, digit1.try_map(|v: &str| v.parse::<usize>())),
    )
    .parse_next(i)
}

fn parse_card<'s>(i: &mut &'s str) -> PResult<(usize, (WinningNumbers, TicketNumbers))> {
    separated_pair(
        parse_card_id,
        terminated(tag(":"), multispace0),
        parse_num_pairs,
    )
    .parse_next(i)
}

fn get_points(i: &str) -> u64 {
    let (_card_id, (winning_numbers, ticket_numbers)) =
        parse_card.parse(i).expect("to parse the card correctly");
    let wins = winning_numbers.intersection(&ticket_numbers).count();

    // part 1
    // match wins {
    //     0 => 0,
    //     _ => 2u64.pow((wins - 1) as u32),
    // }

    // part 2
    return wins as u64;
}

fn main() {
    let result: Vec<(usize, u64)> = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| get_points(line))
        .enumerate()
        .collect();

    // part 2
    let mut options = VecDeque::from(result.clone());
    let mut count = 0;
    while options.len() > 0 {
        let (position, value) = options.pop_front().unwrap();
        count += 1;
        if value == 0 {
            continue;
        }
        let next_index_start = position + 1;
        let next_index = next_index_start + value as usize;
        for i in next_index_start..next_index {
            options.push_back((i, result[i].1));
        }
    }
    println!("{:?}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let mut input = "33 13 28 76 16 91 52 41 38 64";
        let expected = vec![33, 13, 28, 76, 16, 91, 52, 41, 38, 64];
        let output = parse_numbers(&mut input).unwrap();
        assert_eq!(output, expected);

        let mut input = "98 92 96 88 49 10 51  4 15  3";
        let expected = vec![98, 92, 96, 88, 49, 10, 51, 4, 15, 3];
        let output = parse_numbers(&mut input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_num_pairs() {
        let mut input = "33 13 28 76 16 91 52 41 38 64 | 98 92 96 88 49 10 51  4 15  3";
        let expected = (
            WinningNumbers::from_iter(vec![33, 13, 28, 76, 16, 91, 52, 41, 38, 64]),
            TicketNumbers::from_iter(vec![98, 92, 96, 88, 49, 10, 51, 4, 15, 3]),
        );
        let output = parse_num_pairs(&mut input).expect("something went wrong");
        assert_eq!(output, expected);

        let mut input = "41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = (
            WinningNumbers::from_iter(vec![41, 48, 83, 86, 17]),
            TicketNumbers::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
        );
        let output = parse_num_pairs(&mut input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_card_id() {
        let mut input = "Card 1";
        let expected = 1;
        let output = parse_card_id(&mut input).unwrap();
        assert_eq!(output, expected);

        let mut input = "Card   2";
        let expected = 2;
        let output = parse_card_id(&mut input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_card() {
        let mut input = "Card 1: 33 13 28 76 16 91 52 41 38 64 | 98 92 96 88 49 10 51  4 15  3";
        let expected = (
            1,
            (
                WinningNumbers::from_iter(vec![33, 13, 28, 76, 16, 91, 52, 41, 38, 64]),
                TicketNumbers::from_iter(vec![98, 92, 96, 88, 49, 10, 51, 4, 15, 3]),
            ),
        );
        let output = parse_card(&mut input).unwrap();
        assert_eq!(output, expected);

        let mut input = "Card 2: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = (
            2,
            (
                WinningNumbers::from_iter(vec![41, 48, 83, 86, 17]),
                TicketNumbers::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
            ),
        );
        let output = parse_card(&mut input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_get_points() {
        let mut input = "Card 1: 33 13 28 76 16 91 52 41 38 64 | 98 92 96 88 49 10 51  4 15  3";
        let expected = 0;
        let output = get_points(&mut input);
        assert_eq!(output, expected);

        let mut input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = 8;
        let output = get_points(&mut input);
        assert_eq!(output, expected);

        let mut input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let expected = 2;
        let output = get_points(&mut input);
        assert_eq!(output, expected);

        let mut input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let expected = 1;
        let output = get_points(&mut input);
        assert_eq!(output, expected);
    }
}
