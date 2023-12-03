use std::fs;

#[derive(Debug, Clone)]
struct Number {
    value: usize,
    positions: Vec<(usize, usize)>,
}

impl Number {
    fn new(value: usize, positions: Vec<(usize, usize)>) -> Self {
        Self { value, positions }
    }
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
    value: char,
    adjacent: Vec<Number>,
}

impl Symbol {
    fn new(x: usize, y: usize, c: char) -> Self {
        Self { x, y, value: c, adjacent: Vec::new() }
    }
}

fn is_symbol(c: char) -> bool {
    !(c.is_alphanumeric() || c == '.')
}

fn is_adjacent(x: usize, y: usize, to_x: usize, to_y: usize) -> bool {
    let safe_positions = vec![
        (x + 1, y),
        (x, y + 1),
        (x + 1, y + 1),
        (x, y - 1),
        (x - 1, y),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1),
    ];
    let target = (to_x, to_y);
    if safe_positions.contains(&target) {
        return true;
    }

    false
}

fn main() {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (y, line) in fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
    {
        let mut number_char = String::new();
        let mut number_positions: Vec<(usize, usize)> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                number_char.push(c);
                number_positions.push((x, y));
            } else if number_char.len() > 0 {
                let newnum = number_char.parse::<usize>().expect("not a number");
                numbers.push(Number::new(newnum, number_positions));
                number_char = String::new();
                number_positions = Vec::new();
            }

            if is_symbol(c) {
                symbols.push(Symbol::new(x, y, c));
            }
        }
        if number_char.len() > 0 {
            let newnum = number_char.parse::<usize>().expect("not a number");
            numbers.push(Number::new(newnum, number_positions));
        }
    }
    let mut num: i64 = 0;

    for number in numbers.iter() {
        for symbol in symbols.iter_mut() {
            let mut found = false;
            for position in number.positions.iter() {
                if is_adjacent(symbol.x, symbol.y, position.0, position.1) {
                    num += number.value as i64;

                    found = true;
                    symbol.adjacent.push(number.clone());
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    println!("{:?}", num);

    // Part 2
    let s: i64 = symbols.into_iter().map(|v| {
        if v.adjacent.len() > 1 {
            v.adjacent.iter().fold(1, |acc, x| acc * x.value as i64)
        } else {
            return 0;
        }
    }).sum();
    println!("{:?}", s);

}
