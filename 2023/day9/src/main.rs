use std::fs;

fn series(i: &[i128]) -> i128 {
    if i.iter().all(|x| x == &0) {
        return 0;
    }
    let r = i.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i128>>();

    i.last().unwrap() + series(&r)
}

fn part1(i: &str) -> i128 {
    i.lines()
        .map(|x| {
            x.split(" ")
                .map(|v| v.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .map(|x| series(&x))
        .sum()
}

fn prev_series(i: &[i128]) -> i128 {
    if i.iter().all(|x| x == &0) {
        return 0;
    }
    let r = i.windows(2).map(|x| x[0] - x[1]).collect::<Vec<i128>>();

    i.first().unwrap() + prev_series(&r)
}

fn part2(i: &str) -> i128 {
    i.lines()
        .map(|x| {
            x.split(" ")
                .map(|v| v.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .map(|x| prev_series(&x))
        .sum()
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let out = part2(raw.trim());
    println!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series() {
        assert_eq!(series(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(series(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(series(&[10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(
            series(&[
                2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16,
                -17, -18
            ]),
            -19
        );
        assert_eq!(
            series(&[-5, -2, 14, 47, 110, 246, 558, 1248, 2665, 5362, 10162]),
            18233
        );
        assert_eq!(
            series(&[
                14, 38, 84, 172, 346, 686, 1315, 2398, 4130, 6710, 10298, 14952, 20542, 26638,
                32369, 36250, 35974, 28166, 8096, -30652
            ]),
            -96558
        );

        assert_eq!(
            series(&[
                2, 0, -2, -4, -6, -8, -10, -12, -14, -16, -18, -20, -22, -24, -26, -28, -30, -32,
                -34, -36
            ]),
            -38
        );

        assert_eq!(
            series(&[
                -1, 6, 21, 56, 143, 340, 732, 1425, 2531, 4142, 6291, 8898, 11699, 14156, 15346,
                13827, 7479, -6682, -32719, -76020
            ]),
            -143577
        );
    }

    #[test]
    fn test_outlier() {
        assert_eq!(
            series(&[
                12, 38, 85, 159, 263, 397, 558, 740, 934, 1128, 1307, 1453, 1545, 1559, 1468, 1242,
                848, 250, -591, -1717
            ]),
            -3173
        );
    }
}
