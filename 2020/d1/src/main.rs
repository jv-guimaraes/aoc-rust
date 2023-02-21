const INPUT: &str = include_str!("..\\input.txt");

fn part1() {
    let entries: Vec<u32> = INPUT.lines().map(|e| e.parse().unwrap()).collect();
    for e1 in entries.iter() {
        for e2 in entries.iter() {
            if e1 + e2 == 2020 {
                println!("Part 1: {} * {} = {}", e1, e2, e1 * e2);
                return;
            }
        }
    }
}

fn main() {
    part1();
}
