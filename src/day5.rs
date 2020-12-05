#[derive(Debug)]
enum UL {
    Upper,
    Lower,
}

pub fn day5() {
    let input = parse();
    let ids = find_ids(input);

    let ans1 = ids[ids.len() - 1];
    assert_eq!(ans1, 970);

    let ans2 = find_seat(ids).unwrap();
    assert_eq!(ans2, 587);
}

fn find_ids(input: Vec<(Vec<UL>, Vec<UL>)>) -> Vec<usize> {
    let mut vec: Vec<_> = input
        .into_iter()
        .map(|(row_instr, col_instr)| {
            let row = bsearch(&row_instr, 0, 0, 127);
            let col = bsearch(&col_instr, 0, 0, 7);
            row * 8 + col
        })
        .collect();
    vec.sort();
    vec
}

fn find_seat(ids: Vec<usize>) -> Option<usize> {
    for i in 0..ids.len() - 1 {
        if ids[i] + 2 == ids[i + 1] {
            return Some(ids[i] + 1);
        }
    }
    None
}

fn bsearch(input: &Vec<UL>, i: usize, lower: usize, upper: usize) -> usize {
    if lower == upper {
        lower
    } else {
        let mid = (lower + upper) / 2;
        match input[i] {
            UL::Lower => bsearch(input, i + 1, lower, mid),
            UL::Upper => bsearch(input, i + 1, mid + 1, upper),
        }
    }
}

fn parse() -> Vec<(Vec<UL>, Vec<UL>)> {
    std::fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| {
            let row = s[0..s.len() - 3]
                .chars()
                .map(|c| match c {
                    'F' => UL::Lower,
                    'B' => UL::Upper,
                    _ => panic!("Unknown character {}", c),
                })
                .collect();
            let col = s[s.len() - 3..s.len()]
                .chars()
                .map(|c| match c {
                    'L' => UL::Lower,
                    'R' => UL::Upper,
                    _ => panic!("Unknown character {}", c),
                })
                .collect();
            (row, col)
        })
        .collect()
}

#[test]
fn day5_test() {
    day5();
}
