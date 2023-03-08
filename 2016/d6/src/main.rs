use itertools::Itertools;

const INPUT: &str = include_str!("..\\input.txt");

fn most_common(byte_slices: &[&[u8]], index: usize) -> char {
    let dict = byte_slices.iter().map(|x| x[index]).counts();
    dict.into_iter().max_by_key(|x| x.1).unwrap().0 as char
}

fn least_common(byte_slices: &[&[u8]], index: usize) -> char {
    let dict = byte_slices.iter().map(|x| x[index]).counts();
    dict.into_iter().min_by_key(|x| x.1).unwrap().0 as char
}

fn part1(byte_slices: &[&[u8]]) {
    print!("Part 1: ");
    for i in 0..byte_slices[0].len() {
        print!("{}", most_common(byte_slices, i));
    }
    println!()
}

fn part2(byte_slices: &[&[u8]]) {
    print!("Part 2: ");
    for i in 0..byte_slices[0].len() {
        print!("{}", least_common(byte_slices, i));
    }
    println!()
}

fn main()  {
    let byte_slices = INPUT.lines().map(|x| x.as_bytes()).collect_vec();
    part1(&byte_slices);
    part2(&byte_slices);
}