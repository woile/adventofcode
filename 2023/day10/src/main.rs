use std::{char, collections::VecDeque, fs};

type Matrix = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Clone)]
struct Letter {
    x: i32,
    y: i32,
    letter: char,
}

impl Letter {
    fn new(x: i32, y: i32, letter: char) -> Self {
        Self { x, y, letter }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn can_connect(dir: &Direction, next_char: char) -> bool {
    match (dir, next_char) {
        (Direction::North, '|') => true,
        (Direction::South, '|') => true,
        (Direction::East, '-') => true,
        (Direction::West, '-') => true,
        (Direction::North, 'L') => true,
        (Direction::East, 'L') => true,
        (Direction::North, 'J') => true,
        (Direction::West, 'J') => true,
        (Direction::South, '7') => true,
        (Direction::West, '7') => true,
        (Direction::South, 'F') => true,
        (Direction::East, 'F') => true,
        _ => false,
    }
}

// fn dfs(l: Letter, m: &Matrix) -> i32 {

// }

fn bfs(m: Matrix) -> i32 {
    let max_y = m.len();
    let max_x = m[0].len();

    let mut queue = VecDeque::new();
    let mut visited = Vec::new();

    for y in 0..max_y {
        for x in 0..max_x {
            let curr = m[y][x];
            // Found starting position
            if curr == 'S' {
                queue.push_back((Letter::new(x as i32, y as i32, curr), 0));
            }
        }
    }
    let mut max = 0;
    while !queue.is_empty() {
        let (letter, steps) = queue.pop_front().unwrap();
        let (x, y) = (letter.x, letter.y);
        if steps > max {
            max = steps;
        }

        let options = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        for opt in options.iter() {

            let (nx, ny): (i32, i32) = match opt {
                Direction::North => (x, y + 1),
                Direction::South => (x, y - 1),
                Direction::West => (x + 1, y),
                Direction::East => (x - 1, y),
            };

            if nx < 0 || nx >= max_x as i32 || ny < 0 || ny >= max_y as i32 {
                continue;
            }
            let next_char = m[ny as usize][nx as usize];

            if !can_connect(opt, next_char) {
                continue;
            }
            let new_letter = Letter::new(nx, ny, next_char);
            if visited.contains(&new_letter) {
                continue;
            }

            visited.push(new_letter);
            queue.push_back((Letter::new(nx, ny, next_char), steps + 1));
        }
    }
    max
}

fn part1(i: &str) -> i32 {
    let m: Matrix = i.lines().map(|x| x.chars().collect()).collect();
    bfs(m)
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let out = part1(raw.trim());
    println!("{}", out);
}
