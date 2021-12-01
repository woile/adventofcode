use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Measurment {
    count: i32,
    prev: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input.txt")?;

    let parsed_m: Vec<i32> = contents
        .trim()
        .split('\n')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let sum_m: Vec<i32> = parsed_m.windows(3).map(|v| v.iter().sum()).collect();

    let mut m = Measurment {
        count: 0,
        prev: *sum_m.first().unwrap(),
    };
    let measurment = sum_m.iter().fold(m, |acc, next| {
        let count = match next > &acc.prev {
            true => acc.count + 1,
            _ => acc.count,
        };
        m.count = count;
        m.prev = *next;
        m
    });
    println!("Items: {}", measurment.count);
    Ok(())
}
