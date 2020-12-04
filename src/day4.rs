use regex::Regex;
use std::collections::*;

pub fn day4() {
    let passports = parse();

    let valid_passports = find_valid(passports);
    assert_eq!(valid_passports.len(), 254);

    let count = validate(valid_passports);
    dbg!(&count);
}

fn find_valid(input: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    input
        .into_iter()
        .filter(|map| {
            if map.contains_key("cid") {
                map.keys().count() == 8
            } else {
                map.keys().count() == 7
            }
        })
        .collect()
}

fn validate(valid_passports: Vec<HashMap<String, String>>) -> usize {
    valid_passports
        .into_iter()
        .filter(|map| {
            validate_year(map.get("byr").unwrap(), 1920, 2002)
                && validate_year(map.get("iyr").unwrap(), 2010, 2020)
                && validate_year(map.get("eyr").unwrap(), 2020, 2030)
                && validate_height(map.get("hgt").unwrap())
                && validate_hair(map.get("hcl").unwrap())
                && validate_eye(map.get("ecl").unwrap())
                && validate_pid(map.get("pid").unwrap())
        })
        .count()
}

fn validate_year(year: &str, lower: i64, upper: i64) -> bool {
    let year: i64 = year.parse().unwrap();
    year >= lower && year <= upper && year >= 1000 && year <= 9999
}

fn validate_height(height: &str) -> bool {
    if height.ends_with("cm") {
        let height: i64 = height[0..height.len() - 2].parse().unwrap();
        height >= 150 && height <= 193
    } else if height.ends_with("in") {
        let height: i64 = height[0..height.len() - 2].parse().unwrap();
        height >= 59 && height <= 76
    } else {
        false
    }
}

fn validate_hair(color: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(color)
}

fn validate_eye(color: &str) -> bool {
    color == "amb"
        || color == "blu"
        || color == "brn"
        || color == "gry"
        || color == "grn"
        || color == "hzl"
        || color == "oth"
}

fn validate_pid(pid: &str) -> bool {
    let pid_val = pid.parse::<i64>();
    pid.len() == 9 && pid_val.is_ok()
}

fn parse() -> Vec<HashMap<String, String>> {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    let re_split1 = Regex::new(r"\n\n").unwrap();
    let re_split2 = Regex::new(r"[ \n]+").unwrap();
    re_split1
        .split(&input)
        .map(|passport| {
            re_split2
                .split(passport.trim())
                .map(|split| {
                    let vals: Vec<_> = split.split(":").collect();
                    (vals[0].to_owned(), vals[1].to_owned())
                })
                .collect()
        })
        .collect()
}

#[test]
fn day4_test() {
    day4();
}
