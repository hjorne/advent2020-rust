use regex::Regex;
use std::collections::*;

type Rules = HashMap<String, Vec<(usize, String)>>;

pub fn day7() {
    let rules = parse();

    let ans1 = part1(&rules);
    assert_eq!(ans1, 224);

    let ans2 = count_contained("shiny gold", &rules);
    assert_eq!(ans2, 1488);
}

fn part1(rules: &Rules) -> usize {
    rules
        .into_iter()
        .filter(|(bag, _)| can_contain(bag, "shiny gold", &rules))
        .count()
}

fn count_contained(target: &str, rules: &Rules) -> usize {
    dbg!(&target);
    rules
        .get(target)
        .unwrap()
        .into_iter()
        .map(|(count, bag)| count + count * count_contained(bag, rules))
        .sum()
}

fn can_contain(start: &str, target: &str, rules: &Rules) -> bool {
    if rules.get(start).unwrap().is_empty() {
        false
    } else {
        let bags = rules.get(start).unwrap();
        if bags.iter().filter(|(_, bag)| bag == target).count() > 0 {
            true
        } else {
            bags.iter().any(|(_, bag)| can_contain(bag, target, rules))
        }
    }
}

fn parse() -> Rules {
    let re = Regex::new(r"([ \w]+) bags contain (no other bags|([ ,\w]+))\.").unwrap();
    let inner_re = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();

    re.captures_iter(&input)
        .map(|capture| {
            let bag = capture.get(1).unwrap().as_str().to_owned();

            let sec = capture.get(2).unwrap().as_str();
            let contained = if sec == "no other bags" {
                vec![]
            } else {
                inner_re
                    .captures_iter(sec)
                    .map(|inner_capture| {
                        let count = inner_capture.get(1).unwrap().as_str().parse().unwrap();
                        let inner_bag = inner_capture.get(2).unwrap().as_str().to_owned();
                        (count, inner_bag)
                    })
                    .collect()
            };

            (bag, contained)
        })
        .collect()
}

#[test]
fn day7_test() {
    day7();
}
