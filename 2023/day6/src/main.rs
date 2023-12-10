use std::{fs, iter::zip};

fn distance(h: usize, t: usize) -> usize {
    h * (t - h)
}

fn options(t: usize) -> Vec<usize> {
    (1..=t).map(|h| distance(h, t)).collect()
}

fn better_options(r: usize, t: usize) -> usize {
    options(t).iter().filter(|x| *x > &r).count()
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut r = raw
        .trim()
        .lines()
        .map(|x| x.split_at(11).1)
        .map(|x| x.split_whitespace().map(|v| v.parse::<usize>().unwrap()));
    let time = r.next().unwrap();
    let distance = r.next().unwrap();

    let w = zip(time, distance)
        .map(|b| {
            let (t, r) = b;
            better_options(r, t)
        })
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{}", w);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance(1, 10), 9);
        assert_eq!(distance(2, 10), 16);
        assert_eq!(distance(3, 10), 21);
        assert_eq!(distance(4, 10), 24);
        assert_eq!(distance(5, 10), 25);
        assert_eq!(distance(6, 10), 24);
        assert_eq!(distance(7, 10), 21);
        assert_eq!(distance(8, 10), 16);
        assert_eq!(distance(9, 10), 9);
        assert_eq!(distance(10, 10), 0);

        assert_eq!(distance(1, 9), 8);
        assert_eq!(distance(2, 9), 14);
        assert_eq!(distance(3, 9), 18);
        assert_eq!(distance(4, 9), 20);
        assert_eq!(distance(5, 9), 20);
        assert_eq!(distance(6, 9), 18);
        assert_eq!(distance(7, 9), 14);
        assert_eq!(distance(8, 9), 8);
        assert_eq!(distance(9, 9), 0);
    }

    #[test]
    fn test_options() {
        assert_eq!(options(10), vec![9, 16, 21, 24, 25, 24, 21, 16, 9, 0]);
        assert_eq!(options(9), vec![8, 14, 18, 20, 20, 18, 14, 8, 0]);
    }

    #[test]
    fn test_better_options() {
        let out = better_options(246, 55);
        assert_eq!(out, 12);
    }
}
