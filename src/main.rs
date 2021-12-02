use std::{env, fs};

#[path = "day01/solution.rs"] mod day01;
#[path = "day02/solution.rs"] mod day02;
// PATH_MARKER

type PartFunction = fn(String) -> i32;

fn get_day_num() -> u8 {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Must provide a day number as a argument to the program");
    }
    let day_string = args[1].trim().to_string();
    return match day_string.parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Invalid number: {}", day_string);
        }
    };
}

fn get_day_fns() -> (PartFunction, PartFunction) {
    let day_num = get_day_num();
    match day_num {
        1 => return (day01::part1, day01::part2),
        2 => return (day02::part1, day02::part2),
// CASE_MARKER
        _ => {
            panic!("Invalid day number: {}", day_num);
        }
    }
}

fn main() {
    let contents = fs::read_to_string(format!("src/day{:02}/input.txt", get_day_num())).expect("Failed to read puzzle input file");
    let (part1, part2) = get_day_fns();
    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents.clone()));
}
