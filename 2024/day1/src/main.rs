use std::{collections::{BinaryHeap, HashMap}, fs};


fn part1() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut side_a: BinaryHeap<i32> = BinaryHeap::with_capacity(1000);
    let mut side_b: BinaryHeap<i32> = BinaryHeap::with_capacity(1000);
    for line in raw.lines() {
        // println!("{:?}", line.split(" "));
        let mut out = line.split("   ").map(|v| v.parse::<i32>().unwrap());
        side_a.push(out.next().unwrap());
        side_b.push(out.next().unwrap());
    }

    let mut sum = 0;
    while let (Some(a), Some(b)) = (side_a.pop(), side_b.pop()) {
        sum += (a - b).abs();
    }
    println!("{}", sum);
}

fn part2() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut n_count = HashMap::new();
    let numbers: Vec<i32> = raw.lines().map(|line| {
        let mut o = line.split("   ").map(|v| v.parse::<i32>().unwrap());
        let v = o.next().unwrap();
        let c = o.next().unwrap();
        *n_count.entry(c).or_insert(0) += 1;
        return v
    }).collect();

    let r: i32 = numbers.iter().map(|v| {
        v * n_count.get(v).unwrap_or(&0)
    }).sum();

    println!("{}", r);

}

fn main() {
    part2();
}
