// A = Rock
// B = Paper
// C = Scissors
// X = Rock -> 1
// Y = Paper -> 2
// Z = Scissors -> 3
fn play_v1(line: &str) -> u32 {
    match line.split(" ").collect::<Vec<&str>>()[..] {
        ["A", "X"] => 4,
        ["A", "Y"] => 8,
        ["A", "Z"] => 3,
        ["B", "X"] => 1,
        ["B", "Y"] => 5,
        ["B", "Z"] => 9,
        ["C", "X"] => 7,
        ["C", "Y"] => 2,
        ["C", "Z"] => 6,
        _ => 0, // Should never happen..
    }
}

fn part1(input: &String) {
    let result: u32 = input.lines().map(play_v1).sum();
    println!("Part 1: {}", result);
}

// A = Rock
// B = Paper
// C = Scissors
// Rock -> 1
// Paper -> 2
// Scissors -> 3
//
// X -> Lose
// Y -> Draw
// Z -> Win
fn play_v2(line: &str) -> u32 {
    match line.split(" ").collect::<Vec<&str>>()[..] {
        ["A", "X"] => 3,
        ["A", "Y"] => 4,
        ["A", "Z"] => 8,
        ["B", "X"] => 1,
        ["B", "Y"] => 5,
        ["B", "Z"] => 9,
        ["C", "X"] => 2,
        ["C", "Y"] => 6,
        ["C", "Z"] => 7,
        _ => 0, // Should never happen..
    }
}

fn part2(input: &String) {
    let result: u32 = input.lines().map(play_v2).sum();
    println!("Part 2: {}", result);
}

pub fn solve(input: &String) {
    part1(&input);
    part2(&input);
}
