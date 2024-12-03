use std::fs;

use regex::Regex;

fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];

    for (_, [d1, d2]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push(d1.parse::<u64>().unwrap() * d2.parse::<u64>().unwrap());
    }

    results.iter().sum()
}


fn part2(input: &str) -> u64 {
    let re = Regex::new(r"don't\(\)|do\(\)").unwrap();
    let matches = re.find_iter(input);

    let mut curr = None;

    let mut range_start = None;
    let mut new_input: String = String::new();
    for next in matches {
        match curr {
            None => {
                if next.as_str() == "don't()" {
                    curr = Some(next.as_str());
                    range_start = next.range().last();
                    new_input.push_str(&input[..range_start.unwrap()]);
                }
                continue;
            }
            Some(c) => match (c, next.as_str()) {
                ("don't()", "don't()") | ("do()", "do()") => {
                    continue;
                }
                ("don't()", "do()") => {
                    curr = Some(next.as_str());
                    range_start = next.range().last();
                }
                ("do()", "don't()") => {
                    curr = Some(next.as_str());
                    let range_end = next.range().last();
                    new_input.push_str(&input[range_start.unwrap()..range_end.unwrap()]);
                    range_start = range_end;

                }
                _ => unreachable!(),
            },
        }
    }
    if let Some(_curr) = curr {
        if _curr == "do()" {
            new_input.push_str(&input[range_start.unwrap()..]);
        }
    }
    println!("{new_input}");
    return part1(&new_input);
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let r_1 = part1(&raw);
    println!("Part 1: {}", r_1);
    let r_2 = part2(&raw);
    println!("Part 2: {}", r_2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(super::part1(input), 161);
    }

    #[test]
    fn test_sample_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(super::part2(input), 48);
    }
}
