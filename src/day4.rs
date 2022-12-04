fn line_to_pairs(line: &str) -> u32 {
    let ids: Vec<u32> = line
        .split(",")
        .flat_map(|pair| pair.split("-").map(|id| id.parse::<u32>().unwrap()))
        .collect();

    match &ids[..] {
        [s1, e1, s2, e2] => {
            if (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1) {
                return 1;
            }
            return 0;
        }
        _ => 0,
    }
}

fn part1(input: &String) {
    let result: u32 = input.lines().map(line_to_pairs).sum();
    println!("Part 1: {}", result);
}

fn line_to_pairs2(line: &str) -> u32 {
    let ids: Vec<u32> = line
        .split(",")
        .flat_map(|pair| pair.split("-").map(|id| id.parse::<u32>().unwrap()))
        .collect();

    match &ids[..] {
        [s1, e1, s2, e2] => {
            if s1 <= e2 && e1 >= s2 {
                return 1;
            }
            return 0;
        }
        _ => 0,
    }
}

fn part2(input: &String) {
    let result: u32 = input.lines().map(line_to_pairs2).sum();
    println!("Part 2: {}", result);
}

pub fn solve(input: &String) {
    part1(&input);
    part2(&input);
}
