#![allow(unused)]
use itertools::Itertools;
use regex_macro::regex;

const INPUT: &str = include_str!("..\\input.txt");

fn supports_tls(text: &str) -> bool {
    let re = regex!(r"(\[.*?\])");
    for find in re.find_iter(text) {
        let slice = &text[find.start()+1..find.end()-1];
        if supports_tls(slice) {
            return false;
        }
    }
    for (a, b, c, d) in text.bytes().tuple_windows() {
        if a != b && a == d && b == c {
            return true;
        }
    }
    false
}

fn supports_ssl(text: &str) -> bool {
    // whitin brackers
    

    // outside brackets

    todo!()
}

fn main()  {
    let a = "abba[mnop]qrst";
    assert!(supports_tls(a));
    assert!(!supports_tls("abcd[bddb]xyyx"));
    assert!(!supports_tls("aaaa[qwer]tyui"));
    assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));

    let r = INPUT.lines().filter(|s| supports_tls(s)).count();
    println!("Part 1: {}", r);

    let re = regex!(r"(\[.*?\])");
    println!("{}", re.replace_all("abba[mnop]qrst", ""));

}