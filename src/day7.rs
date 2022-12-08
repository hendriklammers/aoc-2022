fn parse_directories(input: &String) -> Vec<u32> {
    let mut temp: Vec<u32> = Vec::new();
    let mut dirs: Vec<u32> = Vec::new();

    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["$", "cd", ".."] => dirs.push(temp.pop().unwrap()),
            ["$", "cd", _] => temp.push(0),
            [size, _] => {
                if let Ok(num) = size.parse::<u32>() {
                    temp.iter_mut().for_each(|n| *n += num)
                }
            }
            [..] => unreachable!(),
        }
    }

    dirs.extend(temp);
    dirs
}

pub fn solve(input: &String) {
    let directories = parse_directories(input);

    let part1: u32 = directories.iter().filter(|&&size| size < 100_000).sum();
    println!("Part 1: {}", part1);

    let largest = directories.iter().max().unwrap();
    let required_size = largest + 30_000_000 - 70_000_000;

    let part2 = directories
        .iter()
        .filter(|&&dir| dir >= required_size)
        .min()
        .unwrap();

    println!("Part 2: {}", part2);
}
