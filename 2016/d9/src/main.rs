#![allow(unused)]
#![allow(clippy::needless_range_loop)]
use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

fn decompress(text: &str) -> String {
    let mut result = String::new();
    let chars = text.chars().collect_vec();
    
    let mut i = 0;
    while i < chars.len() {
        // Not a compression sequence
        if chars[i] != '(' {
            result.push(chars[i]);
            i += 1;
            continue;
        }
        
        // Find closing parentheses
        let mut closing_parentheses_ix = 0;
        for j in (i+1).. {
            if chars[j] == ')' {
                closing_parentheses_ix = j;
                break;
            }
        }

        // Calculate marker and sequence range and times to repeat (1x3)
        let marker = &text[i+1..closing_parentheses_ix];
        let range_size: usize = marker.split('x').next().unwrap().parse().unwrap();
        let range = (closing_parentheses_ix+1)..(closing_parentheses_ix+1+range_size);
        let times: usize  = marker.split('x').nth(1).unwrap().parse().unwrap();

        // Write slices to result string
        for _ in 0..times {
            result.push_str(&text[range.clone()]);
        }

        // Calculate the next i
        i = closing_parentheses_ix + 1 + range_size;
    }
    result
}

fn main()  {
    println!("Part 1: {}", decompress(INPUT.trim()).len());
}

#[test]
fn no_compression_sequence_test() {
    assert_eq!("ADVENT".to_owned(), decompress("ADVENT"));
}

#[test]
fn compression_sequence_whitout_inner_marker() {
    assert_eq!("ABBBBBC".to_owned(), decompress("A(1x5)BC"));
    assert_eq!("XYZXYZXYZ".to_owned(), decompress("(3x3)XYZ"));
    assert_eq!("ABCBCDEFEFG".to_owned(), decompress("A(2x2)BCD(2x2)EFG"));
}

#[test]
fn compression_with_inner_marker() {
    assert_eq!("(1x3)A".to_owned(), decompress("(6x1)(1x3)A"));
    assert_eq!("X(3x3)ABC(3x3)ABCY".to_owned(), decompress("X(8x2)(3x3)ABCY"));
}