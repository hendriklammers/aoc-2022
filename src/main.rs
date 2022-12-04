mod day4;
use std::env;
use std::fs;

fn read_input(day_number: u8) -> String {
    let file_path = env::current_dir()
        .unwrap()
        .join("inputs")
        .join(format!("day-{}.txt", day_number));

    fs::read_to_string(file_path).expect("Unable to open input file")
}

fn main() {
    day4::solve(&read_input(4))
}
