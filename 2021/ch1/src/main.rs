use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Measurment {
    count: i32,
    prev: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input.txt")?;
    let measurments: Vec<&str> = contents.trim().split('\n').collect();
    let initial_measurment = measurments
        .first()
        .expect("No initial value")
        .parse::<i32>()
        .unwrap();
    let mut m = Measurment {
        count: 0,
        prev: initial_measurment,
    };
    let measur = measurments
        .iter()
        .map(|v| v.parse::<i32>().unwrap())
        .fold(m, |acc, next| {
            let count = match next > acc.prev {
                true => acc.count + 1,
                _ => acc.count
            };
            m.count = count;
            m.prev = next;
            m
        });
    println!("Items: {}", measur.count);
    Ok(())
}
