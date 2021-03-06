use std::{env, fs};
use aoc_2021::*;

type PartFunction = fn(String) -> i64;

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
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),
        6 => (day06::part1, day06::part2),
        7 => (day07::part1, day07::part2),
        8 => (day08::part1, day08::part2),
        9 => (day09::part1, day09::part2),
        10 => (day10::part1, day10::part2),
        11 => (day11::part1, day11::part2),
        12 => (day12::part1, day12::part2),
        13 => (day13::part1, day13::part2),
        14 => (day14::part1, day14::part2),
        15 => (day15::part1, day15::part2),
        16 => (day16::part1, day16::part2),
        17 => (day17::part1, day17::part2),
// CASE_MARKER
        _ => {
            panic!("Invalid day number: {}", day_num);
        }
    }
}

fn main() {
    let contents = fs::read_to_string(format!("src/day{:02}/input.txt", get_day_num())).expect("Failed to read puzzle input file");
    let (part1, part2) = get_day_fns();
    let start_part1 = std::time::Instant::now();
    println!("Part 1: {} ({} ms)", part1(contents.clone()), start_part1.elapsed().as_millis());
    let start_part2 = std::time::Instant::now();
    println!("Part 2: {} ({} ms)", part2(contents), start_part2.elapsed().as_millis());
}
