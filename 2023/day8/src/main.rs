use std::{collections::HashMap, fs};

fn parse_lines(i: &str) -> HashMap<&str, (&str, &str)> {
    i.lines()
        .map(|l| {
            (
                l.get(0..3).unwrap(),
                (l.get(7..10).unwrap(), l.get(12..15).unwrap()),
            )
        })
        .collect()
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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
                }
                'R' => {
                    current_loc = s.get(&current_loc).unwrap().1;
                    steps += 1;
                }
                _ => unreachable!("Letter not found"),
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

fn part2(i: &str) -> usize {
    let (instructions, last) = i.split_once("\n\n").unwrap();
    let s = parse_lines(last);

    let ops: Vec<_> = s.keys().filter(|k| k.ends_with("A")).collect();

    let mut steps_comb = Vec::with_capacity(ops.len());
    for o in ops.iter() {
        let mut current_loc = **o;
        let mut steps = 0;
        loop {
            for dir in instructions.chars() {
                match dir {
                    'L' => {
                        current_loc = s.get(&current_loc).unwrap().0;
                        steps += 1;
                    }
                    'R' => {
                        current_loc = s.get(&current_loc).unwrap().1;
                        steps += 1;
                    }
                    _ => unreachable!("Letter not found"),
                };
                if current_loc.ends_with("Z") {
                    break;
                }
            }
            if current_loc.ends_with("Z") {
                break;
            }
        }
        steps_comb.push(steps);
    }
    lcm(&steps_comb)
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let out = part2(raw.trim());
    println!("{out}");
}
