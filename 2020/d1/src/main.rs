const INPUT: &str = include_str!("..\\input.txt");

fn part1(entries: &[u32]) {
    for e1 in entries.iter() {
        for e2 in entries.iter() {
            if e1 + e2 == 2020 {
                println!("Part 1: {} * {} = {}", e1, e2, e1 * e2);
                return;
            }
        }
    }
}

fn part2(entries: &[u32]) {
    for e1 in entries.iter() {
        for e2 in entries.iter() {
            for e3 in entries.iter() {
                if e1 + e2 + e3 == 2020 {
                    println!("Part 2: {} * {} * {} = {}", e1, e2, e3, e1 * e2 * e3);
                    return;
                }
            }
        }
    }
}

fn main() {
    let entries: Vec<u32> = INPUT.lines().map(|e| e.parse().unwrap()).collect();
    part1(&entries);
    part2(&entries);
}
