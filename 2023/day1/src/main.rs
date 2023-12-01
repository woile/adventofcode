use std::fs;

fn main() {
    let result: i32 = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            for l in line.chars() {
                if l.is_digit(10) {
                    if first.is_none() {
                        first = Some(l);
                    } else {
                        last = Some(l);
                    }
                }
            }
            let r = match (first, last) {
                (Some(firstv), Some(lastv)) => {
                    format!("{}{}", firstv, lastv).parse::<i32>().unwrap()
                }
                (Some(firstv), None) => format!("{}{}", firstv, firstv).parse::<i32>().unwrap(),
                (None, Some(lastv)) => format!("{}{}", lastv, lastv).parse::<i32>().unwrap(),
                _ => 0,
            };
            println!("{}", r);
            r
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    println!("{}", result);
}
