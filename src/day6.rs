use regex::Regex;
use std::collections::*;

pub fn day6() {
    let input = parse();

    let ans1 = part1(&input);
    assert_eq!(ans1, 6335);

    let ans2 = part2(&input);
    assert_eq!(ans2, 3392);
}

fn part1(input: &Vec<Vec<String>>) -> usize {
    input
        .into_iter()
        .map(|questions| count_yes(questions))
        .sum()
}

fn part2(input: &Vec<Vec<String>>) -> usize {
    input
        .into_iter()
        .map(|questions| count_all_yes(questions))
        .sum()
}

fn count_yes(questions: &Vec<String>) -> usize {
    questions
        .into_iter()
        .flat_map(|question_str| question_str.chars())
        .collect::<HashSet<char>>()
        .len()
}

fn count_all_yes(questions: &Vec<String>) -> usize {
    let mut counter = HashMap::new();
    questions
        .into_iter()
        .flat_map(|question_str| question_str.chars())
        .for_each(|c| *counter.entry(c).or_insert(0) += 1);

    counter
        .into_iter()
        .filter(|(_, val)| *val == questions.len())
        .count()
}

fn parse() -> Vec<Vec<String>> {
    let re = Regex::new(r"\n\n").unwrap();

    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();

    re.split(&input)
        .map(|lines| lines.split_whitespace().map(|s| s.to_owned()).collect())
        .collect()
}

#[test]
fn day6_test() {
    day6();
}
