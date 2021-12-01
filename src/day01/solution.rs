pub fn part1(input: String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|line| line.trim().parse().unwrap()).collect();
    return depths
        .windows(2)
        .map(|pair| pair[0] < pair[1])
        .filter(|increase| *increase == true)
        .collect::<Vec<bool>>()
        .len()
        .try_into()
        .unwrap();
}

pub fn part2(input: String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|line| line.trim().parse().unwrap()).collect();
    return depths
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|pair| pair[0] < pair[1])
        .filter(|increase| *increase == true)
        .collect::<Vec<bool>>()
        .len()
        .try_into()
        .unwrap();
}
