use regex::Regex;

#[derive(Debug)]
struct Pass {
    lower: usize,
    upper: usize,
    t: char,
    chars: Vec<char>,
}

pub fn day2() {
    let input = parse();

    let ans1 = part1(&input);
    assert_eq!(ans1, 458);

    let ans2 = part2(&input);
    assert_eq!(ans2, 342);
}

fn part1(input: &Vec<Pass>) -> usize {
    input
        .into_iter()
        .filter(|pass| {
            let count = pass.chars.iter().filter(|&&c| c == pass.t).count();
            count >= pass.lower && count <= pass.upper
        })
        .count()
}

fn part2(input: &Vec<Pass>) -> usize {
    input
        .into_iter()
        .filter(|pass| {
            let bool1 = pass.chars[pass.lower - 1] == pass.t;
            let bool2 = pass.chars[pass.upper - 1] == pass.t;
            bool1 ^ bool2
        })
        .count()
}

fn parse() -> Vec<Pass> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    re.captures_iter(&input)
        .map(|capture| {
            let lower = capture.get(1).unwrap().as_str().parse().unwrap();
            let upper = capture.get(2).unwrap().as_str().parse().unwrap();
            let t = capture.get(3).unwrap().as_str().chars().nth(0).unwrap();
            let chars = capture.get(4).unwrap().as_str().chars().collect();

            Pass {
                lower,
                upper,
                t,
                chars,
            }
        })
        .collect()
}

#[test]
fn day2_test() {
    day2();
}
