use std::collections::HashMap;

fn part2(input: &str) -> i64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (left_part, right_part) = line.split_once("   ").unwrap();
        left.push(left_part.parse::<i64>().unwrap());
        right.push(right_part.parse::<i64>().unwrap());
    }

    let frequencies = right
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, val|{
            map.entry(val)
                .and_modify(|frq|*frq+=1)
                .or_insert(1);
            map
        });
    left
        .iter()
        .map(|a| frequencies.get(a).unwrap_or_else(|| {&0i64}) * a)
        .sum::<i64>()
}

fn main() {
    let input = include_str!("part1.txt");
    let dist = part2(input);
    println!("dist: {}", dist);
}
