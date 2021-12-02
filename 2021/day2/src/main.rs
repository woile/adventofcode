use std::io::{self, BufRead};

struct Dive {
    depth: i32,
    horizontal: i32,
    aim: i32,
}
impl Dive {
    fn new() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }
}
fn main() {
    let directions: Vec<String> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .collect();
    let mut dive = Dive::new();

    // Part 1
    directions.iter().for_each(|direction| {
        let pair: Vec<&str> = direction.split(' ').collect();
        let cmd = pair.first().unwrap();
        let num = pair.last().unwrap().parse::<i32>().unwrap();
        match *cmd {
            "forward" => dive.horizontal += num,
            "down" => dive.depth += num,
            "up" => dive.depth -= num,
            _ => unreachable!("not this"),
        }
    });

    let horizontal_depth = dive.depth * dive.horizontal;
    println!("final horizontal position by your final depth is: {}", horizontal_depth);

    let mut dive = Dive::new();
    // Part 2
    directions.iter().for_each(|direction| {
        let pair: Vec<&str> = direction.split(' ').collect();
        let cmd = pair.first().unwrap();
        let num = pair.last().unwrap().parse::<i32>().unwrap();
        match *cmd {
            "forward" => {
                dive.horizontal += num;
                dive.depth += dive.aim * num;

            },
            "down" => dive.aim += num,
            "up" => dive.aim -= num,
            _ => unreachable!("not this"),
        }
    });
    let horizontal_depth = dive.depth * dive.horizontal;
    println!("[WITH AIM] final horizontal position by your final depth is: {}", horizontal_depth);
}
