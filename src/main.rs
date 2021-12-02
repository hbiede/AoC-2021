use std::{env, fs};

#[path = "day01/solution.rs"] mod day01;
// PATH_MARKER

type PartFunction = fn(String) -> i32;

fn get_day_fns() -> (PartFunction, PartFunction) {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Must provide a day number as a argument to the program");
    }
    let day_string = args[1].trim().to_string();
    let day_num: u8 = match day_string.parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Invalid number: {}", day_string);
        }
    };
    match day_num {
        1 => return (day01::part1, day01::part2),
// CASE_MARKER
        _ => {
            panic!("Invalid day number: {}", day_num);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("src/day01/input.txt").expect("Failed to read puzzle input file");
    let (part1, part2) = get_day_fns();
    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents.clone()));
}
