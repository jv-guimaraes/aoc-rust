#![allow(unused)]

const INPUT: u32 = 347991;

fn part1() {
    for (i, j) in (3..).step_by(2).zip(1..) {
        let floor = (i-2) * (i-2) + 1;
        let ceil = i * i;
        let mid = j-1 + floor;
        let step = i - 1;
        // println!("{j} | {i} | {floor}-{ceil} | mid:{},{},{},{}", mid, mid+step, mid+step*2, mid+step*3);

        if (floor..ceil).contains(&INPUT) { // input case
            let mut distance = INPUT.abs_diff(mid);
            for m in &[mid, mid+step, mid+step*2, mid+step*3] {
                // println!("{} - {} = {}",INPUT, m, INPUT.abs_diff(*m));
                if INPUT.abs_diff(*m) < distance {
                    distance = INPUT.abs_diff(*m);
                }
            }
            println!("Part 1: i + distance = {}", j + distance);
            return;
        }
    }
}

fn main() {
    part1();
}
