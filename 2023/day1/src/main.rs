use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{peek, value},
    multi::many0,
    IResult,
};

fn parse_str_num(i: &str) -> IResult<&str, &str> {
    let (tail, out) = peek(alt((
        value("1", tag("one")),
        value("2", tag("two")),
        value("3", tag("three")),
        value("4", tag("four")),
        value("5", tag("five")),
        value("6", tag("six")),
        value("7", tag("seven")),
        value("8", tag("eight")),
        value("9", tag("nine")),
    )))(i)?;
    let (tail, _) = take(1usize)(tail)?;
    Ok((tail, out))
}
fn parse_num(i: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((
        take_while1(move |c: char| c.is_numeric()),
        parse_str_num,
        value("", take(1usize)),
    )))(i)
}

fn parser(i: &str) -> i32 {
    println!("{}", i);
    let (_, num) = parse_num(i).expect("numer not returned");
    let binding = num.join("");
    let mut chars = binding.trim().chars();
    println!("{:?}", chars);
    let first = chars.next().unwrap_or('0');
    let last = chars.next_back().unwrap_or(first);
    let val = format!("{}{}", first, last).parse::<i32>().unwrap();
    println!("{}", val);
    println!("-----------------");
    val
}

fn main() {
    let result: i32 = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| parser(line))
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    println!("{}", result);
}
