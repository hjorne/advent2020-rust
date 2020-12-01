use std::collections::*;

pub fn day1() -> Option<()> {
    let input = parse();

    let ans1 = two_sum(&input, 2020)?;
    assert_eq!(ans1, 181044);

    let ans2 = three_sum(&input)?;
    assert_eq!(ans2, 82660352);

    Some(())
}

fn two_sum(input: &Vec<i64>, target: i64) -> Option<i64> {
    let mut seen = HashSet::new();

    for n in input {
        let to_find = target - n;
        if seen.contains(&to_find) {
            return Some(n * to_find);
        } else {
            seen.insert(n);
        }
    }

    None
}

fn three_sum(input: &Vec<i64>) -> Option<i64> {
    for i in input {
        if let Some(n) = two_sum(input, 2020 - i) {
            return Some(i * n);
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
