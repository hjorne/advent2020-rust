use num::integer::lcm;
use ring_algorithm::chinese_remainder_theorem as crt;

pub fn day13() {
    let (timestamp, buses) = parse();

    let ans1 = part1(timestamp, &buses);
    assert_eq!(ans1, 119);

    let ans2 = part2(&buses);
    assert_eq!(ans2, 1106724616194525);
}

fn part1(timestamp: i64, buses: &[Option<i64>]) -> i64 {
    let timestamp_float = timestamp as f64;

    let (bus, diff) = buses
        .into_iter()
        .filter_map(|obus| {
            obus.map(|bus| {
                let arrival = bus * (timestamp_float / (bus as f64)).ceil() as i64;
                (bus, arrival - timestamp)
            })
        })
        .min_by(|(_, diff1), (_, diff2)| diff1.cmp(diff2))
        .unwrap();

    bus * diff
}

fn part2(buses: &[Option<i64>]) -> i64 {
    let coeffs: Vec<_> = buses
        .into_iter()
        .enumerate()
        .filter_map(|(i, obus)| obus.map(|bus| (-(i as i64), bus)))
        .collect();

    let lcm = coeffs.iter().fold(1, |x, (_, y)| lcm(x, *y));
    let (u, m): (Vec<_>, Vec<_>) = coeffs.into_iter().unzip();
    let rem = crt(&u, &m).unwrap();
    let mult = (rem as f64) / (lcm as f64);

    rem + lcm * (mult.floor().abs() as i64)
}

fn parse() -> (i64, Vec<Option<i64>>) {
    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();

    let lines: Vec<_> = input.split_whitespace().collect();
    let buses = lines[1]
        .split(",")
        .map(|s| match s {
            "x" => None,
            _ => Some(s.parse().unwrap()),
        })
        .collect();
    (lines[0].parse().unwrap(), buses)
}

#[test]
fn day13_test() {
    day13();
}
