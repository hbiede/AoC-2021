use std::collections::HashMap;

fn point_count(input: String, diagonals: bool) -> i32 {
    let mut points = HashMap::new();
    input
        .lines()
        .for_each(|line| {
            let point_strings = line.split(" -> ").collect::<Vec<&str>>();
            let a = point_strings[0].split(',').collect::<Vec<&str>>();
            let b = point_strings[1].split(',').collect::<Vec<&str>>();
            let ((ax, ay), (bx, by)) = (
                (a[0].parse::<i32>().unwrap(), a[1].parse::<i32>().unwrap()),
                (b[0].parse::<i32>().unwrap(), b[1].parse::<i32>().unwrap()),
            );
            if ax == bx {
                // Vertical
                for y in std::cmp::min(ay, by)..=std::cmp::max(ay, by) {
                    *(points.entry((ax, y)).or_insert(0)) += 1;
                }
            } else if ay == by {
                // Horizontal
                for x in std::cmp::min(ax, bx)..=std::cmp::max(ax, bx) {
                    *(points.entry((x, ay)).or_insert(0)) += 1;
                }
            } else if diagonals && ((ay - by) / (ax - bx)).abs() == 1 {
                // Diagonals
                let m = (ay - by) / (ax - bx);
                let b = ay - m * ax;
                for x in std::cmp::min(ax, bx)..=std::cmp::max(ax, bx) {
                    *(points.entry((x, m * x + b)).or_insert(0)) += 1;
                }
            }
        });
    points.into_values().filter(|&x| x > 1).count() as i32
}

pub fn part1(input: String) -> i32 {
    point_count(input, false)
}

pub fn part2(input: String) -> i32 {
    point_count(input, true)
}
