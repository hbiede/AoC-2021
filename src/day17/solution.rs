use std::cmp::{max, min};
use regex::Regex;

pub fn part1(input: String) -> i64 {
    let y_min: f64 = Regex::new(r"y=(-?\d+)")
        .unwrap()
        .captures(&input)
        .unwrap()[1]
        .parse()
        .unwrap();
    (((y_min.abs() - 0.5).powf(2.0)) / 2.0).round() as i64
}

pub fn part2(input: String) -> i64 {
    let re_match = Regex::new(r".*x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")
        .unwrap()
        .captures(&input)
        .unwrap();
    let x_min: i64 = re_match[1].parse().unwrap();
    let x_max: i64 = re_match[2].parse().unwrap();
    let y_a: i64 = re_match[3].parse().unwrap();
    let y_b: i64 = re_match[4].parse().unwrap();
    let y_max: i64 = max(y_a, y_b);
    let y_min: i64 = min(y_a, y_b);

    let hits = |init_x_velocity: i64, init_y_velocity: i64| {
        let mut x_velocity = init_x_velocity;
        let mut y_velocity = init_y_velocity;
        let mut x = 0;
        let mut y = 0;
        loop {
            if x > x_max {
                // overshot
                return false;
            }
            if x_velocity <= 0 && x < x_min {
                // Stopped before the target
                return false;
            }
            if y_velocity <= 0 && y < y_min {
                // Too slow
                return false;
            }

            if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
                return true;
            }

            x += x_velocity;
            y += y_velocity;

            x_velocity = max(0, x_velocity - 1);
            y_velocity -= 1;
        }
    };

    let mut working_combos = 0;
    let max_y_velocity = max(y_a.abs(), y_b.abs());
    for x_velocity in 1..=x_max {
        for y_velocity in (-max_y_velocity)..=max_y_velocity {
            if (x_velocity != 0 || y_velocity != 0) && hits(x_velocity, y_velocity) {
                working_combos += 1;
            }
        }
    }

    working_combos
}
