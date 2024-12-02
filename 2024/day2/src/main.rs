#![feature(control_flow_enum)]
use std::{fs, ops::ControlFlow};

#[derive(Debug)]
enum Direction {
    None,
    Increase,
    Decrease,
}

fn is_safe_increase(curr: usize, prev: usize) -> bool {
    curr > prev && (curr - prev > 0 && curr - prev < 4)
}

fn is_safe_decrease(curr: usize, prev: usize) -> bool {
    curr < prev && prev - curr > 0 && prev - curr < 4
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            report
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap())
                .try_fold((Direction::None, None), |(dir, prev), curr| {
                    match (dir, prev) {
                        (Direction::None, None) => {
                            ControlFlow::Continue((Direction::None, Some(curr)))
                        }
                        (Direction::None, Some(v)) => {
                            if is_safe_increase(curr, v) {
                                ControlFlow::Continue((Direction::Increase, Some(curr)))
                            } else if is_safe_decrease(curr, v) {
                                ControlFlow::Continue((Direction::Decrease, Some(curr)))
                            } else {
                                ControlFlow::Break(())
                            }
                        }
                        (Direction::Increase, Some(v)) => {
                            if is_safe_increase(curr, v) {
                                ControlFlow::Continue((Direction::Increase, Some(curr)))
                            } else {
                                ControlFlow::Break(())
                            }
                        }
                        (Direction::Decrease, Some(v)) => {
                            if is_safe_decrease(curr, v) {
                                ControlFlow::Continue((Direction::Decrease, Some(curr)))
                            } else {
                                ControlFlow::Break(())
                            }
                        }
                        _ => unreachable!(),
                    }
                })
                .continue_value()
                .is_some_and(|(dir, _)| match dir {
                    Direction::None => false,
                    _ => true,
                })
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            let values: Vec<usize> = report
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            let length = values.len();
            for i in 0..length {
                let mut index = 0;
                let v = values
                    .iter()
                    .try_fold((Direction::None, None), |(dir, prev), curr| {
                        if i == index {
                            index += 1;
                            return ControlFlow::Continue((dir, prev));
                        }
                        index += 1;
                        match (dir, prev) {
                            (Direction::None, None) => {
                                ControlFlow::Continue((Direction::None, Some(*curr)))
                            }
                            (Direction::None, Some(v)) => {
                                if is_safe_increase(*curr, v) {
                                    ControlFlow::Continue((Direction::Increase, Some(*curr)))
                                } else if is_safe_decrease(*curr, v) {
                                    ControlFlow::Continue((Direction::Decrease, Some(*curr)))
                                } else {
                                    ControlFlow::Break(())
                                }
                            }
                            (Direction::Increase, Some(v)) => {
                                if is_safe_increase(*curr, v) {
                                    ControlFlow::Continue((Direction::Increase, Some(*curr)))
                                } else {
                                    ControlFlow::Break(())
                                }
                            }
                            (Direction::Decrease, Some(v)) => {
                                if is_safe_decrease(*curr, v) {
                                    ControlFlow::Continue((Direction::Decrease, Some(*curr)))
                                } else {
                                    ControlFlow::Break(())
                                }
                            }
                            _ => unreachable!(),
                        }
                    })
                    .continue_value()
                    .is_some_and(|(dir, _)| match dir {
                        Direction::None => false,
                        _ => true,
                    });
                if v == true {
                    return true;
                }
            }
            return false;
        })
        .count()
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let r_1 = part1(&raw);
    println!("Part 1: {}", r_1);
    let r_2 = part2(&raw);
    println!("Part 2: {}", r_2);
}
