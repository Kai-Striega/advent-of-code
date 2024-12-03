const MAX_IGNOREABLE_REPORTS: u8 = 1;

fn is_strictly_increasing(v: &Vec<i64>) -> bool {
    v.iter()
        .zip(v.iter().skip(1))
        .map(|(a, b)| a < b)
        .all(|x| x)
}

fn is_strictly_decreasing(v: &Vec<i64>) -> bool {
    v.iter()
        .zip(v.iter().skip(1))
        .map(|(a, b)| a > b)
        .all(|x| x)
}

fn is_slowly_changing(v: &Vec<i64>) -> bool {
    v.iter()
        .zip(v.iter().skip(1))
        .map(|(a, b)| i64::abs(a - b) < 4)
        .all(|x| x)
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect()
        })
        .filter(|v| is_strictly_increasing(v) || is_strictly_decreasing(v))
        .filter(|v| is_slowly_changing(v))
        .collect::<Vec<_>>()
        .len()
}

fn main() {
    let input = include_str!("part1.txt");
    let p1 = part1(input);
    println!("{}", p1);
}
