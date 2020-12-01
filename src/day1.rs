use std::collections::*;

pub fn day1() -> Option<()> {
    let input = parse();

    let ans1 = part1(&input)?;
    assert_eq!(ans1, 181044);

    let ans2 = part2(&input)?;
    assert_eq!(ans2, 82660352);

    Some(())
}

fn part1(input: &Vec<i64>) -> Option<i64> {
    let mut seen = HashSet::new();

    for n in input {
        let target = 2020 - n;
        if seen.contains(&target) {
            return Some(n * target);
        } else {
            seen.insert(n);
        }
    }

    None
}

fn part2(input: &Vec<i64>) -> Option<i64> {
    let mut seen = HashSet::new();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let target = 2020 - input[i] - input[j];
            if seen.contains(&target) {
                return Some(input[i] * input[j] * target);
            } else {
                seen.insert(input[i]);
            }
        }
    }

    None
}

fn parse() -> Vec<i64> {
    std::fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[test]
fn day1_test() {
    let ans = day1();
    assert!(ans.is_some());
}
