// #![allow(unused)]

use std::{collections::HashMap};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref DICT: Mutex<HashMap<String, u16>> = Mutex::new(HashMap::new());
}

fn insert_into_dict(identifier: &str, value: u16) {
    let mut dict = DICT.lock().unwrap();
    dict.insert(identifier.to_owned(), value);
    // println!("{} = {} ({})", identifier, value, dict.len());
}

fn value_of(commands: &str, identifier: &str) -> u16 {
    if DICT.lock().unwrap().contains_key(identifier) {
        return *DICT.lock().unwrap().get(identifier).unwrap();
    }

    for line in commands.lines().rev() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let reciever = *tokens.last().unwrap();
        
        if reciever != identifier { continue; }
        
        if tokens.len() == 3 {
            if let Ok(num) = tokens[0].parse::<u16>() {
                insert_into_dict(identifier, num);
                return num;
            } else {
                return value_of(commands, tokens[0])
            }
        }
        else if line.contains("NOT") {
            let operand = tokens[1];
            if let Ok(num) = operand.parse::<u16>() {
                insert_into_dict(identifier, !num);
                return !num;
            } else {
                return !value_of(commands, operand)
            }
        }
        else if line.contains("AND") {
            let (a, b) = (tokens[0], tokens[2]);
            let a = a.parse().unwrap_or_else(|_| value_of(commands, a));
            let b = b.parse().unwrap_or_else(|_| value_of(commands, b));
            insert_into_dict(identifier, a & b);
            return a & b;
        }
        else if line.contains("OR") {
            let (a, b) = (tokens[0], tokens[2]);
            let a = a.parse().unwrap_or_else(|_| value_of(commands, a));
            let b = b.parse().unwrap_or_else(|_| value_of(commands, b));
            insert_into_dict(identifier, a | b);
            return a | b;
        }
        else if line.contains("LSHIFT") {
            let (a, b) = (tokens[0], tokens[2]);
            if let ( Ok(a), Ok(b) ) = ( a.parse::<u16>(), b.parse::<u16>() ) {
                insert_into_dict(identifier, a << b);
                return a << b;
            } else {
                return value_of(commands, a) << b.parse::<u16>().unwrap();
            }
        }
        else if line.contains("RSHIFT") {
            let (a, b) = (tokens[0], tokens[2]);
            if let ( Ok(a), Ok(b) ) = ( a.parse::<u16>(), a.parse::<u16>() ) {
                insert_into_dict(identifier, a >> b);
                return a >> b;
            } else {
                return value_of(commands, a) >> b.parse::<u16>().unwrap();
            }
        }
    }
    
    panic!("yabe");
}

fn _test() {
    let input = include_str!("..\\smaller_input.txt");
    dbg!(value_of(input, "k"));
    dbg!(value_of(input, "d"));
    dbg!(value_of(input, "e"));
    dbg!(value_of(input, "f"));
    dbg!(value_of(input, "g"));
    dbg!(value_of(input, "h"));
    dbg!(value_of(input, "i"));
    dbg!(value_of(input, "x"));
    dbg!(value_of(input, "y"));
}

fn main() {
    let commands = include_str!("..\\input.txt");
    dbg!(value_of(commands, "a"));
    DICT.lock().unwrap().clear();
    
    let commands2 = commands.replace("14146", "956");
    dbg!(value_of(&commands2, "a"));
}


