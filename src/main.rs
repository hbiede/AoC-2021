use std::fs;
#[path = "day01/solution.rs"] mod day01;

fn main() {
    let contents = fs::read_to_string("src/day01/input.txt").expect("Failed to read puzzle input file");
    println!("Part 1: {}", day01::part1(contents.clone()));
    println!("Part 2: {}", day01::part2(contents.clone()));
}
