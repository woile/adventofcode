use std::{fs, collections::HashMap};

fn parse_lines(i: &str) -> HashMap<&str, (&str, &str)> {
    i.lines().map(|l| {
        (l.get(0..3).unwrap(), (l.get(7..10).unwrap(), l.get(12..15).unwrap()))
    }).collect()
}
fn part1(i: &str) -> i64 {
    let (instructions, last) = i.split_once("\n\n").unwrap();
    let s = parse_lines(last);

    let mut steps = 0;

    let mut current_loc = "AAA";

    loop {
        for dir in instructions.chars() {
            match dir {
                'L' => {
                    current_loc = s.get(&current_loc).unwrap().0;
                    steps += 1;
                },
                'R' => {
                    current_loc = s.get(&current_loc).unwrap().1;
                    steps += 1;
                },
                _ => unreachable!("Letter not found")
            };
            if current_loc == "ZZZ" {
                break;
            }
        }
        if current_loc == "ZZZ" {
            break;
        }
    }

    return steps;
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let out = part1(raw.trim());
    println!("{out}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(""), 1);
    }
}