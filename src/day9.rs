use super::day1::two_sum;

pub fn day9() {
    let input = parse();

    let ans1 = part1(&input);
    assert_eq!(ans1.unwrap(), 1124361034);

    let ans2 = part2(&input, ans1.unwrap());
    assert_eq!(ans2.unwrap(), 129444555);
}

fn part1(input: &Vec<i64>) -> Option<i64> {
    for i in 25..input.len() {
        if let None = two_sum(&input[i - 25..i], input[i]) {
            return Some(input[i]);
        }
    }

    None
}

fn part2(input: &Vec<i64>, target: i64) -> Option<i64> {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i..j].into_iter().sum::<i64>() == target {
                let min = input[i..j].into_iter().min().unwrap();
                let max = input[i..j].into_iter().max().unwrap();
                return Some(min + max);
            }
        }
    }

    None
}

fn parse() -> Vec<i64> {
    std::fs::read_to_string("inputs/day9.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[test]
fn day9_test() {
    day9();
}
