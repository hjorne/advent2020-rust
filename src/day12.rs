use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
enum Instr {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

pub fn day12() {
    let instrs = parse();

    let ans1 = part1(&instrs);
    assert_eq!(ans1, 845);

    let ans2 = part2(&instrs);
    assert_eq!(ans2, 27016);
}

fn part1(instrs: &Vec<Instr>) -> i64 {
    let mut heading = (1, 0);
    let mut pos = (0, 0);

    for &instr in instrs {
        match instr {
            Instr::North(val) => pos.1 += val,
            Instr::South(val) => pos.1 -= val,
            Instr::East(val) => pos.0 += val,
            Instr::West(val) => pos.0 -= val,
            Instr::Left(val) => heading = rotate(val, heading),
            Instr::Right(val) => heading = rotate(-val, heading),
            Instr::Forward(val) => {
                pos.0 += val * heading.0;
                pos.1 += val * heading.1;
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn part2(instrs: &Vec<Instr>) -> i64 {
    let mut waypoint = (10, 1);
    let mut pos = (0, 0);

    for &instr in instrs {
        match instr {
            Instr::North(val) => waypoint.1 += val,
            Instr::South(val) => waypoint.1 -= val,
            Instr::East(val) => waypoint.0 += val,
            Instr::West(val) => waypoint.0 -= val,
            Instr::Left(val) => waypoint = rotate(val, waypoint),
            Instr::Right(val) => waypoint = rotate(-val, waypoint),
            Instr::Forward(val) => {
                pos.0 += val * waypoint.0;
                pos.1 += val * waypoint.1;
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn rotate(theta: i64, (x, y): (i64, i64)) -> (i64, i64) {
    let theta_float = theta as f64 * (PI / 180.0);
    let x_float = x as f64;
    let y_float = y as f64;

    let x_new = (x_float * theta_float.cos() - y_float * theta_float.sin()).round() as i64;
    let y_new = (x_float * theta_float.sin() + y_float * theta_float.cos()).round() as i64;

    (x_new, y_new)
}

fn parse() -> Vec<Instr> {
    std::fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| {
            let val = s[1..].parse().unwrap();
            match &s[0..1] {
                "N" => Instr::North(val),
                "S" => Instr::South(val),
                "E" => Instr::East(val),
                "W" => Instr::West(val),
                "L" => Instr::Left(val),
                "R" => Instr::Right(val),
                "F" => Instr::Forward(val),
                _ => panic!("Unknown type"),
            }
        })
        .collect()
}

#[test]
fn day12_test() {
    day12();
}
