use std::collections::HashMap;

pub fn day10() {
    let input = parse();

    let ans1 = part1(&input);
    assert_eq!(ans1, 2343);

    let ans2 = part2(&input);
    assert_eq!(ans2, 31581162962944);
}

fn part1(jolts: &Vec<i64>) -> i64 {
    let mut diff1 = 0;
    let mut diff3 = 0;
    for i in 0..jolts.len() - 1 {
        if jolts[i + 1] - jolts[i] == 1 {
            diff1 += 1;
        } else {
            diff3 += 1;
        }
    }

    diff1 * diff3
}

fn part2(jolts: &Vec<i64>) -> i64 {
    let graph = gen_graph(&jolts);
    let mut count = HashMap::new();

    for jolt in jolts {
        let sum = graph
            .get(jolt)
            .map(|children| {
                children
                    .into_iter()
                    .map(|child| count.get(child).unwrap_or(&0))
                    .fold(0, |x, &y| x + y)
            })
            .unwrap_or(1);

        *count.entry(*jolt).or_insert(0) += sum;
    }

    *count.get(&jolts[jolts.len() - 1]).unwrap()
}

fn gen_graph(jolts: &Vec<i64>) -> HashMap<i64, Vec<i64>> {
    let mut graph = HashMap::new();

    for i in 0..jolts.len() {
        for j in 0..i {
            if jolts[i] - jolts[j] <= 3 {
                graph.entry(jolts[i]).or_insert(vec![]).push(jolts[j]);
            }
        }
    }

    graph
}

fn parse() -> Vec<i64> {
    let mut vec: Vec<_> = std::fs::read_to_string("inputs/day10.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    vec.push(0);
    vec.sort();
    vec.push(vec[vec.len() - 1] + 3);
    vec
}

#[test]
fn day10_test() {
    day10();
}
