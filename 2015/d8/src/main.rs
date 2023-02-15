#![allow(unused)]

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input: &str = include_str!("..\\input.txt");
    let char_count = input.lines().map(|l| l.len()).sum::<usize>();
    let byte_count = input.lines().map(count_bytes).sum::<usize>();
    println!("{} - {} = {}", char_count, byte_count, char_count - byte_count);

    // part 2
    let encoding_count = input.lines().map(encoding_size).sum::<usize>();
    println!("{} - {} = {}", encoding_count, char_count, encoding_count - char_count);
}

fn count_bytes(s: &str) -> usize {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"\\\\").unwrap();
        static ref RE2: Regex = Regex::new(r#"\\""#).unwrap();
        static ref RE3: Regex = Regex::new(r#"\\x.."#).unwrap();
    }

    let s = &s[1..s.len()-1];
    let s = RE1.replace_all(s, "0");
    let s = RE2.replace_all(&s, "0");
    let s = RE3.replace_all(&s, "0");
    // println!("{} ==> {}", s, &res);

    s.len()
}

fn encoding_size(s: &str) -> usize {
    let s = s.replace('\\', "\\\\");
    let s = s.replace('"', "\\\"");
    s.len() + 2 // + 2 for the outer quote pair
}