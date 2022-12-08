mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
use std::env;
use std::fs;
use std::process;

fn read_input(day_number: u8) -> String {
    let file_path = env::current_dir()
        .unwrap()
        .join("inputs")
        .join(format!("day-{}.txt", day_number));

    fs::read_to_string(file_path).expect("Unable to open input file")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a day number");
        process::exit(1);
    }

    match args[1].parse().unwrap() {
        1 => day1::solve(&read_input(1)),
        2 => day2::solve(&read_input(2)),
        3 => day3::solve(&read_input(3)),
        4 => day4::solve(&read_input(4)),
        5 => day5::solve(&read_input(5)),
        6 => day6::solve(&read_input(6)),
        7 => day7::solve(&read_input(7)),
        _ => println!("Not a valid day"),
    }
}
