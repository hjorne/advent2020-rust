use regex::Regex;
use std::collections::*;

#[derive(Debug, Copy, Clone)]
enum Instr {
    Nop(i64),
    Jmp(i64),
    Acc(i64),
}

pub fn day8() {
    let instrs = parse();

    let ans1 = part1(&instrs);
    assert_eq!(ans1, 1337);

    let ans2 = part2(&instrs);
    assert_eq!(ans2.unwrap(), 1358);
}

fn part1(instrs: &Vec<Instr>) -> i64 {
    let mut executed: HashSet<i64> = HashSet::new();
    let mut acc = 0;
    let mut instr_ptr = 0;

    while !executed.contains(&instr_ptr) {
        executed.insert(instr_ptr);
        match instrs[instr_ptr as usize] {
            Instr::Nop(_) => instr_ptr += 1,
            Instr::Jmp(val) => instr_ptr += val,
            Instr::Acc(val) => {
                acc += val;
                instr_ptr += 1;
            }
        }
    }
    acc
}

fn part2(instrs: &Vec<Instr>) -> Option<i64> {
    for i in 0..instrs.len() {
        let new_op = match instrs[i] {
            Instr::Nop(val) => Instr::Jmp(val),
            Instr::Jmp(val) => Instr::Nop(val),
            Instr::Acc(_) => continue,
        };

        let mut new_instrs = instrs.clone();
        new_instrs[i] = new_op;

        if let Some(n) = run_correct(&new_instrs) {
            return Some(n);
        }
    }

    None
}

fn run_correct(instrs: &Vec<Instr>) -> Option<i64> {
    let mut executed: HashSet<i64> = HashSet::new();
    let mut acc = 0;
    let mut instr_ptr = 0;

    loop {
        executed.insert(instr_ptr);
        match instrs[instr_ptr as usize] {
            Instr::Nop(_) => instr_ptr += 1,
            Instr::Jmp(val) => instr_ptr += val,
            Instr::Acc(val) => {
                acc += val;
                instr_ptr += 1;
            }
        }

        if executed.contains(&instr_ptr) {
            break None;
        } else if instr_ptr as usize == instrs.len() {
            break Some(acc);
        }
    }
}

fn parse() -> Vec<Instr> {
    let re = Regex::new(r"(nop|acc|jmp) (-|\+)(\d+)").unwrap();
    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();

    re.captures_iter(&input)
        .map(|capture| {
            let instr_type = capture.get(1).unwrap().as_str();
            let sgn = capture.get(2).unwrap().as_str();
            let digits: i64 = capture.get(3).unwrap().as_str().parse().unwrap();

            let val: i64 = if sgn == "-" { -digits } else { digits };

            match instr_type {
                "nop" => Instr::Nop(val),
                "jmp" => Instr::Jmp(val),
                "acc" => Instr::Acc(val),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

#[test]
fn day8_test() {
    day8();
}
