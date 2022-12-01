use std::env;
use std::fs;

fn read_file() -> String {
    let file_path = env::current_dir().unwrap().join("inputs/day-1.txt");
    fs::read_to_string(file_path).expect("Unable to open input file")
}

fn main() {
    let input = read_file();

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
    println!("solution: {}", most);
}
