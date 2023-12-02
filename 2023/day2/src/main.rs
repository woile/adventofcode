use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Game(usize);

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Debug, PartialEq)]
enum Token {
    Red(usize),
    Blue(usize),
    Green(usize),
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    preceded(
        tag("Game"),
        terminated(
            preceded(
                multispace0,
                map(take_while1(|c| char::is_digit(c, 10)), |v: &str| {
                    Game(v.parse::<usize>().unwrap())
                }),
            ),
            tag(":"),
        ),
    )(i)
}

// 4 red, 8 blue, 3 green
fn parse_set(i: &str) -> IResult<&str, CubeSet> {
    let (tail, tokens) = separated_list1(
        tag(","),
        alt((
            terminated(
                delimited(
                    multispace0,
                    map(take_while1(|c| char::is_digit(c, 10)), |v: &str| {
                        Token::Red(v.parse::<usize>().unwrap())
                    }),
                    multispace0,
                ),
                tag("red"),
            ),
            terminated(
                delimited(
                    multispace0,
                    map(take_while1(|c| char::is_digit(c, 10)), |v: &str| {
                        Token::Blue(v.parse::<usize>().unwrap())
                    }),
                    multispace0,
                ),
                tag("blue"),
            ),
            terminated(
                delimited(
                    multispace0,
                    map(take_while1(|c| char::is_digit(c, 10)), |v: &str| {
                        Token::Green(v.parse::<usize>().unwrap())
                    }),
                    multispace0,
                ),
                tag("green"),
            ),
        )),
    )(i)?;
    let set = tokens.iter().fold(
        CubeSet {
            red: 0,
            blue: 0,
            green: 0,
        },
        |acc, x| match x {
            Token::Red(v) => CubeSet {
                red: *v,
                blue: acc.blue,
                green: acc.green,
            },
            Token::Blue(v) => CubeSet {
                red: acc.red,
                blue: *v,
                green: acc.green,
            },
            Token::Green(v) => CubeSet {
                red: acc.red,
                blue: acc.blue,
                green: *v,
            },
            _ => acc,
        },
    );
    Ok((tail, set))
}

// 1 green, 7 red; 1 green, 9 red, 3 blue; 4 blue, 5 red
fn parse_cube_sets(i: &str) -> IResult<&str, Vec<CubeSet>> {
    separated_list1(tag(";"), delimited(multispace0, parse_set, multispace0))(i)
}

fn parse_game_samples(i: &str) -> IResult<&str, (Game, Vec<CubeSet>)> {
    pair(
        parse_game,
        delimited(
            multispace0,
            parse_cube_sets,
            multispace0,
        ),
    )(i)
}

fn parser(i: &str) -> Option<usize> {
    let (_, (game, cube_sets)) = parse_game_samples(i).unwrap();

    let is_out_of_bound = cube_sets
        .iter()
        .any(|x| x.red > 12 || x.green > 13 || x.blue > 14);

    if is_out_of_bound {
        return None;
    }
    Some(game.0)
}

fn main() {
    let result: usize = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| parser(line))
        .flatten()
        .sum();
    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 1:";
        let expected = Game(1);

        let (_, output) = parse_game(input).unwrap();
        assert_eq!(output, expected);

        let input = "Game 99:";
        let expected = Game(99);

        let (_, output) = parse_game(input).unwrap();
        assert_eq!(output, expected);

        let input = "Game 100:";
        let expected = Game(100);

        let (_, output) = parse_game(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_set() {
        let input = "4 red, 8 blue, 3 green";
        let expected = CubeSet {
            red: 4,
            blue: 8,
            green: 3,
        };

        let (_, output) = parse_set(input).unwrap();
        assert_eq!(output, expected);

        let input = "2 green, 12 blue";
        let expected = CubeSet {
            red: 0,
            blue: 12,
            green: 2,
        };

        let (_, output) = parse_set(input).unwrap();
        assert_eq!(output, expected);

        let input = "8 green";
        let expected = CubeSet {
            red: 0,
            blue: 0,
            green: 8,
        };

        let (_, output) = parse_set(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_cube_sets() {
        let input = "1 green, 7 red; 1 green, 9 red, 3 blue; 4 blue, 5 red";
        let expected = vec![
            CubeSet {
                red: 7,
                blue: 0,
                green: 1,
            },
            CubeSet {
                red: 9,
                blue: 3,
                green: 1,
            },
            CubeSet {
                red: 5,
                blue: 4,
                green: 0,
            },
        ];

        let (_, output) = parse_cube_sets(input).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_game_samples() {
        let input = "Game 1: 2 green, 12 blue; 6 red, 6 blue; 8 blue, 5 green, 5 red";
        let expected = (
            Game(1),
            vec![
                CubeSet {
                    red: 0,
                    blue: 12,
                    green: 2,
                },
                CubeSet {
                    red: 6,
                    blue: 6,
                    green: 0,
                },
                CubeSet {
                    red: 5,
                    blue: 8,
                    green: 5,
                },
            ],
        );
        let (_ , output) = parse_game_samples(input).unwrap();
        assert_eq!(output, expected);
    }
}
