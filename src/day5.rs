use std::collections::HashMap;

enum Solution {
    Part1,
    Part2,
}

fn create_stacks(input: &String) -> HashMap<u32, Vec<String>> {
    let mut stacks = HashMap::<u32, Vec<String>>::new();

    for line in input.lines().take(8) {
        for (index, letter) in line.chars().skip(1).step_by(4).enumerate() {
            if letter.to_string() != " " {
                let values = stacks.entry((index + 1) as u32).or_insert(vec![]);
                values.insert(0, letter.to_string());
            }
        }
    }

    return stacks;
}

struct Move {
    amount: u32,
    from: u32,
    to: u32,
}

fn line_to_move(line: &str) -> Move {
    let moves = line
        .trim()
        .split(" ")
        .filter_map(|word| match word.parse::<u32>() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect::<Vec<u32>>();

    return Move {
        amount: moves[0],
        from: moves[1],
        to: moves[2],
    };
}

fn solve_part(input: &String, part: Solution) -> String {
    let mut stacks = create_stacks(input);

    let moves: Vec<Move> = input.lines().skip(10).map(line_to_move).collect();

    for m in moves {
        if matches!(part, Solution::Part1) {
            // Move one by one
            for _ in 0..m.amount {
                let from = stacks.entry(m.from).or_default();
                let crate_to_move = from.pop().unwrap();
                let to = stacks.entry(m.to).or_default();
                to.push(crate_to_move);
            }
        } else if matches!(part, Solution::Part2) {
            // Move stacks
            let from = stacks.entry(m.from).or_default();
            let start_range: usize = from.len() - (m.amount as usize);
            let mut crates_to_move = from.drain(start_range..).collect();
            let to = stacks.entry(m.to).or_default();
            to.append(&mut crates_to_move);
        }
    }

    let mut result = String::new();
    for index in 1..=9 {
        let last_crate = &stacks[&index].last().unwrap();
        result.push_str(last_crate);
    }

    return result;
}

pub fn solve(input: &String) {
    let part1 = solve_part(input, Solution::Part1);
    println!("Part 1: {}", part1);

    let part2 = solve_part(input, Solution::Part2);
    println!("Part 2: {}", part2);
}
