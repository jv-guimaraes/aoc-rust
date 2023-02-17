#![allow(unused)]

use std::collections::HashSet;

const INPUT: &str = include_str!("..\\input.txt");

fn part1() {
    let mut total = 0;
    for num in INPUT.split_ascii_whitespace() {
        total += num.parse::<i32>().unwrap();
    }
    println!("Part 1: {total}")
}

fn part2() {
    let mut frequencies: HashSet<i32> = HashSet::new();
    frequencies.insert(0);
    let mut total = 0;
    for num in INPUT.split_ascii_whitespace().cycle() {
        let num = num.parse::<i32>().unwrap();
        total += num;
        // println!("Total: {}", total);
        if !frequencies.insert(total) {
            println!("Part 2: {}", total);
            return;
        }
    }
}

fn main() {
    part1();
    part2();
}
