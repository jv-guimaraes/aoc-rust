use std::collections::HashMap;

const INPUT: &str = include_str!("..\\input.txt");

fn contains_repeated_char(line: &str, times: i32) -> bool {
    let mut dict: HashMap<char, i32> = HashMap::new();
    for char in line.chars() {
        dict.entry(char).and_modify(|c| *c += 1).or_insert(1);
    }
    dict.into_values().any(|count| count == times)
}

fn part1() {
    let mut twice = 0;
    let mut three_times = 0;
    for line in INPUT.lines() {
        if contains_repeated_char(line, 2) {
            twice += 1
        }
        if contains_repeated_char(line, 3) {
            three_times += 1;
        }
    }
    println!("Part 1: {} * {} = {}", twice, three_times, twice * three_times);
}

fn char_diff(line1: &str, line2: &str) -> u32 {
    let mut diff = 0;
    for (c1, c2) in line1.chars().zip(line2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
    }
    diff
}

fn part2() {
    let lines: Vec<&str> = INPUT.lines().collect();
    for i in 0..lines.len()-1 {
        for j in i+1..lines.len() {
            if char_diff(lines[i], lines[j]) == 1 {
                let mut result = String::new();
                for (c1, c2) in lines[i].chars().zip(lines[j].chars()) {
                    if c1 == c2 {
                        result.push(c1);
                    }
                }
                println!("Part 2: {result}");
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
