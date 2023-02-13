#[macro_use]
extern crate lazy_static;

use fancy_regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Problem 1: {}", problem_1(&input));
    println!("Problem 2: {}", problem_2(&input));
}

fn problem_1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| acc + is_nice(line) as u32)
}

fn problem_2(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| acc + is_really_nice(line) as u32)
}

fn is_nice(s: &str) -> bool {
    lazy_static! {
        static ref R1: Regex = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
        static ref R2: Regex = Regex::new(r"(\w)\1").unwrap();
        static ref R3: Regex = Regex::new(r"ab|cd|pq|xy").unwrap();
    }
    R1.is_match(s).unwrap() && R2.is_match(s).unwrap() && !R3.is_match(s).unwrap()
}

fn is_really_nice(s: &str)  -> bool {
    lazy_static! {
        static ref R1: Regex = Regex::new(r"(\w\w).*\1").unwrap();
        static ref R2: Regex = Regex::new(r"(\w)\w\1").unwrap();
    }
    R1.is_match(s).unwrap() && R2.is_match(s).unwrap()
}
