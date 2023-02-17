fn part1(input: &str) {
    let mut valid_triangles = 0;
    for triangle in input.lines() {
        let sides: Vec<i32> = triangle
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if sides[0] >= sides[1] + sides[2] { continue; }
        if sides[1] >= sides[0] + sides[2] { continue; }
        if sides[2] >= sides[0] + sides[1] { continue; }

        valid_triangles += 1;
    }
    println!("Part 1: {valid_triangles}");
}

fn part2(input: &str) {
    let input: Vec<i32> = input.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut valid_triangles = 0;
    
    for i in (0..input.len()).step_by(9) {
        for j in 0..3 {
            let sides = [input[i+j], input[i+3+j], input[i+6+j]];
    
            if sides[0] >= sides[1] + sides[2] { continue; }
            if sides[1] >= sides[0] + sides[2] { continue; }
            if sides[2] >= sides[0] + sides[1] { continue; }
    
            valid_triangles += 1;
        }
    }
    
    println!("Part 2: {valid_triangles}");
}

fn main() {
    let input = include_str!("..\\input.txt");
    part1(input);
    part2(input);
}
