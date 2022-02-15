use std::cmp::min;
use std::collections::vec_deque::VecDeque;

fn parse_base(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}
fn get_new_risk(risk: usize, x: usize, y: usize) -> usize {
    let mut new_risk = risk + x + y;
    while new_risk >= 10 {
        new_risk -= 9;
    }
    new_risk
}
fn parse_chiton(input: String, is_repeating: bool) -> Vec<Vec<usize>> {
    let base = parse_base(input);
    if is_repeating {
        let mut result: Vec<Vec<usize>> = base.to_vec();
        for vertical_repeat in 0..5 {
            for vertical_index in 0..base.len() {
                let row = vertical_repeat * base.len() + vertical_index;
                if row >= result.len() {
                    result.push(vec![]);
                }
                for horizontal_repeat in 0..5 {
                    if horizontal_repeat == vertical_repeat && vertical_repeat == 0 {
                        continue;
                    }
                    for horizontal in &(base[vertical_index].clone()) {
                        result[row].push(get_new_risk(*horizontal, horizontal_repeat, vertical_repeat));
                    }
                }
            }
        }
        result
    } else {
        base
    }
}

fn find_risk_level(input: String, is_repeating: bool) -> i64 {
    let risk_levels: Vec<Vec<usize>> = parse_chiton(input, is_repeating);
    println!("{}", risk_levels.iter().map(|row| row.iter().map(|cell| format!("{}", cell)).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("\n"));

    let mut risk_level_totals: Vec<Vec<i64>> = risk_levels.iter().map(|row| {
        row.iter().map(|_| {
            0
        }).collect::<Vec<i64>>()
    }).collect::<Vec<Vec<i64>>>();
    let mut queue = VecDeque::from(vec![(1, 0), (0, 1)]);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current.0 == 0 {
            risk_level_totals[current.0][current.1] = risk_levels[current.0][current.1] as i64 + risk_level_totals[current.0][current.1 - 1];
        } else if current.1 == 0 {
            risk_level_totals[current.0][current.1] = risk_levels[current.0][current.1] as i64 + risk_level_totals[current.0 - 1][current.1];
        } else {
            risk_level_totals[current.0][current.1] = risk_levels[current.0][current.1] as i64 + min(risk_level_totals[current.0 - 1][current.1], risk_level_totals[current.0][current.1 - 1]);
        }

        if current.0 + 1 < risk_level_totals.len() && risk_level_totals[current.0 + 1][current.1] == 0 && !queue.contains(&(current.0 + 1, current.1)) {
            queue.push_back((current.0 + 1, current.1));
        }
        if current.1 + 1 < risk_level_totals[0].len() && risk_level_totals[current.0][current.1 + 1] == 0 && !queue.contains(&(current.0, current.1 + 1)) {
            queue.push_back((current.0, current.1 + 1));
        }
    }
    risk_level_totals[risk_level_totals.len() - 1][risk_level_totals[risk_level_totals.len() - 1].len() - 1]
}

pub fn part1(input: String) -> i64 {
    find_risk_level(input, false)
}

pub fn part2(input: String) -> i64 {
    find_risk_level(input, true)
}
