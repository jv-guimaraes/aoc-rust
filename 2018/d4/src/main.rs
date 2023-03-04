#![allow(unused)]
mod util;

use itertools::Itertools;
use time::{Date, PrimitiveDateTime};
use util::*;

const INPUT: &str = include_str!("..\\input.txt");

#[derive(Debug)]
enum Mind {
    Sleep,
    Awake,
    Undefined,
}

#[derive(Debug)]
struct Day {
    date: Date,
    id: u32,
    minutes: [Mind; 60],
}

fn main() {
    let mut logs = INPUT.lines().collect_vec();
    for log in logs {
        println!("{log}");
    }
}
