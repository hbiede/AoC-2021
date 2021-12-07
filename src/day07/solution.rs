pub fn part1(input: String) -> i64 {
    let mut positions = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    // Unstable sort performs faster on primitive types
    positions.sort_unstable();
    // Move to the median value
    let dest = positions[(positions.len() as f64 / 2_f64).floor() as usize];
    positions.iter().map(|x| (x - dest).abs()).sum()
}

fn find_increasing_fuel(positions: Vec<i64>, dest: i64)-> i64 {
    positions.iter().map(|x| ((x - dest).abs() * ((x - dest).abs() + 1)) / 2).sum::<i64>()
}

pub fn part2(input: String) -> i64 {
    let positions = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    // Move to the average value, plus or minus 1
    let dest = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let ceil = dest.ceil() as i64;
    let floor = dest.floor() as i64;
    *[
        find_increasing_fuel(positions.clone(), ceil),
        find_increasing_fuel(positions, floor)
    ].iter().min().unwrap()
}
