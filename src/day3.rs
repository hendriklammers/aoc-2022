fn get_points(letter: char) -> u32 {
    if letter.is_uppercase() {
        return (letter as u32) - 38;
    }
    return (letter as u32) - 96;
}

fn check_items(str: &str) -> u32 {
    let half = str.len() / 2;
    let chars: Vec<char> = str.chars().collect();

    for char in &chars[half..] {
        if chars[..half].contains(char) {
            return get_points(char.clone());
        }
    }
    return 0;
}

fn part1(input: &String) {
    let result: u32 = input.lines().map(check_items).sum();
    println!("Part 1: {}", result);
}

fn part2(input: &String) {
    let result: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            // It works lol
            for char in group[0].chars().collect::<Vec<char>>() {
                if group[1].chars().collect::<Vec<char>>().contains(&char)
                    && group[2].chars().collect::<Vec<char>>().contains(&char)
                {
                    return get_points(char);
                }
            }
            return 0;
        })
        .sum();
    println!("Part 2: {}", result);
}

pub fn solve(input: &String) {
    part1(&input);
    part2(&input);
}
