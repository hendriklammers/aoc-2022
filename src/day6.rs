fn check_unique(window: &[u8]) -> bool {
    !window
        .iter()
        .enumerate()
        .any(|(index, character)| window[..index].contains(character))
}

fn find_marker(input: &String, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .position(check_unique)
        .unwrap()
        + size
}

pub fn solve(input: &String) {
    let result1 = find_marker(input, 4);
    println!("Part 1: {}", result1);

    let result2 = find_marker(input, 14);
    println!("Part 2: {}", result2);
}
