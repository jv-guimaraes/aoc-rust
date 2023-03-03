// #![allow(unused)]

use std::{collections::HashMap, fmt::{Debug, Display}};

use regex::Regex;
use lazy_static::lazy_static;

const INPUT: &str = include_str!("..\\input.txt");
// const INPUT: &str = include_str!("..\\sample.txt");

#[derive(Debug)]
struct Room<'a> {   
    name: &'a str,
    id: u32,
    checksum: &'a str,
}

impl Display for Room<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}|{} {:?}",  self.name, self.id, self.checksum, self.is_real())
    }
}

impl Room<'_> {
    fn from_str(text: &str) -> Room {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"^([a-z]+-)+").unwrap();
            static ref ID_RE: Regex = Regex::new(r"\d+").unwrap();
            static ref CHECKSUM_RE: Regex = Regex::new(r"\w+\]").unwrap();
        }
        let name = NAME_RE.find(text).unwrap().as_str();
        let id = ID_RE.find(text).unwrap().as_str().parse().unwrap();
        let checksum = CHECKSUM_RE.find(text).unwrap().as_str().strip_suffix(']').unwrap();
        Room { name, id, checksum }
    }

    fn letter_count(&self) -> HashMap<char, u32> {
        let mut counter: HashMap<char, u32> = HashMap::new();
        for c in self.name.chars().filter(|c| c.is_ascii_lowercase()) {
            counter.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        counter
    }

    fn is_real(&self) -> bool {
        let mut vetor = self.letter_count().into_iter().collect::<Vec<_>>();
        vetor.sort_by_key(|a| a.0);
        vetor.sort_by_key(|a| std::cmp::Reverse(a.1));
        let vetor: Vec<char> = vetor.into_iter().take(5).map(|x| x.0).collect();
        let checksum: Vec<char> = self.checksum.chars().collect();
        vetor == checksum
    }
}

fn main()  {
    let rooms = INPUT.lines().map(Room::from_str);
    let result: u32 = rooms.filter(|r| r.is_real()).fold(0, |acc, x| acc + x.id);
    println!("{result}");
}