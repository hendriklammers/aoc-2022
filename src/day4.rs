enum Solution {
    Part1,
    Part2,
}

fn check_overlap(line: &str, part: Solution) -> bool {
    let ids: Vec<u32> = line
        .split(",")
        .flat_map(|pair| pair.split("-").map(|id| id.parse::<u32>().unwrap()))
        .collect();

    match &ids[..] {
        [s1, e1, s2, e2] => match part {
            Solution::Part1 => (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1),
            Solution::Part2 => s1 <= e2 && e1 >= s2,
        },
        _ => false,
    }
}

fn part1(input: &String) {
    let result: u32 = input
        .lines()
        .filter(|l| check_overlap(l, Solution::Part1))
        .collect::<Vec<&str>>()
        .len() as u32;
    println!("Part 1: {}", result);
}

fn part2(input: &String) {
    let result: u32 = input
        .lines()
        .filter(|l| check_overlap(l, Solution::Part2))
        .collect::<Vec<&str>>()
        .len() as u32;
    println!("Part 2: {}", result);
}

pub fn solve(input: &String) {
    part1(&input);
    part2(&input);
}
