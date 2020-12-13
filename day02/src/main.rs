#![warn(clippy::all)]
#![allow(clippy::needless_return)]

use lazy_static::lazy_static;

use regex::Regex;
use std::fs;

struct PasswordLine {
    pos_a: usize,
    pos_b: usize,
    letter: char,
    password: String,
}

fn parse_line(line: &str) -> PasswordLine {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<min>\d+)\-(?P<max>\d+) (?P<char>\w): (?P<password>\w+)").unwrap();
    }

    let caps = RE.captures(line).unwrap();

    return PasswordLine {
        pos_a: caps[1].parse().unwrap(),
        pos_b: caps[2].parse().unwrap(),
        letter: caps[3].chars().next().unwrap(),
        password: caps[4].to_string(),
    };
}

fn matches_first_rule(pw_line: &PasswordLine) -> bool {
    let count = pw_line.password.matches(pw_line.letter).count();
    return (pw_line.pos_a..=pw_line.pos_b).contains(&count);
}

fn matches_second_rule(pw_line: &PasswordLine) -> bool {
    let nth_char = |n: usize| -> bool {
        pw_line.password.chars().nth(n - 1).unwrap_or('\0') == pw_line.letter
    };
    return nth_char(pw_line.pos_a) ^ nth_char(pw_line.pos_b);
}

fn main() {
    let lines = fs::read_to_string("./input.txt").expect("Could not load the input :(");

    let mut first_rule_count = 0;
    let mut second_rule_count = 0;

    for line in lines.split('\n') {
        let pw_line = parse_line(line);
        first_rule_count += matches_first_rule(&pw_line) as i32;
        second_rule_count += matches_second_rule(&pw_line) as i32;
    }

    println!(
        "{} passwords are valid according to the first rule.",
        first_rule_count
    );
    println!(
        "{} passwords are valid according to the second rule.",
        second_rule_count
    );
}
