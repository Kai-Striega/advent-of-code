fn part1(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (left_part, right_part) = line.split_once("   ").unwrap();
        left.push(left_part);
        right.push(right_part);
    }

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right.iter())
        .map(|(a, b)| a.parse::<i64>().unwrap() - b.parse::<i64>().unwrap())
        .map(|x| x.abs())
        .sum::<i64>()
}

fn main() {
    let input = include_str!("part1.txt");
    let sum_of_diffs = part1(input);
    println!("{sum_of_diffs}");
}

