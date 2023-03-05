const RANGE: std::ops::RangeInclusive<u32> = 271973..=785961;

fn is_valid(password: u32) -> bool {
    let bytes = password.to_string().into_bytes();

    // Going from left to right, the digits never decrease;
    if bytes.windows(2).any(|x| x[0] > x[1]) {
        return false;
    }

    // Two adjacent digits are the same (like 22 in 122345)
    if !bytes.windows(2).any(|x| x[0] == x[1]) {
        return false;
    }

    true
}

fn is_valid2(password: u32) -> bool {
    let b = password.to_string().into_bytes();

    // Going from left to right, the digits never decrease;
    if b.windows(2).any(|x| x[0] > x[1]) {
        return false;
    }

    if b[0] == b[1] && b[1] != b[2] { return true }
    if b[1] == b[2] && b[1] != b[0] && b[1] != b[3] { return true }
    if b[2] == b[3] && b[2] != b[1] && b[2] != b[4] { return true }
    if b[3] == b[4] && b[3] != b[2] && b[3] != b[5] { return true }
    if b[4] == b[5] && b[4] != b[3] { return true }

    false
}

fn main() {
    println!("Part1: {}", RANGE.filter(|x| is_valid(*x)).count());
    println!("Part2: {}", RANGE.filter(|x| is_valid2(*x)).count());
}

#[test]
fn is_valid_test() {
    assert!(is_valid(444444));
    assert!(is_valid(222222));
    assert!(is_valid(345567));
    assert!(!is_valid(345678));
    assert!(!is_valid(455464));
    assert!(!is_valid(323450));
    assert!(!is_valid(345678));
}
