fn part1(input: &String) {
    let mut most = 0;
    let mut current = 0;

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(amount) => {
                current = current + amount;
            }
            Err(_) => {}
        }
        if line == "" {
            if current > most {
                most = current;
            }
            current = 0;
        }
    }
    println!("Part 1: {}", most);
}

fn part2(input: &String) {
    let mut totals = vec![];
    let mut current = 0;

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(amount) => {
                current = current + amount;
            }
            Err(_) => {}
        }
        if line == "" {
            totals.push(current);
            current = 0;
        }
    }

    totals.sort();
    totals.reverse();

    let result: i32 = (&totals[..3]).iter().sum();

    println!("Part 2: {}", result);
}

pub fn solve(input: &String) {
    part1(&input);
    part2(&input);
}
