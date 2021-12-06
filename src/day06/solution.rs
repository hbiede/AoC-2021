fn fish_count(input: String, days: usize) -> i64 {
    let mut fish_memo = vec![];
    let fish = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for i in 0..=7 {
        // Fill with starter fish's spawn counts for the appropriate day
        fish_memo.push(fish.iter().filter(|x| **x == i as i32).count() as i64);
    }
    let mut fish_count = fish.len() as i64;
    let mut delayed_fish = vec![0; 7];
    for i in 0..days {
        // Increase based on the fish that split a week ago (plus those that were spawned 8 days
        // ago)
        fish_count += fish_memo[i % 7];
        // Update fish spawn counts for the appropriate day with the fish that were born 8 days ago
        fish_memo[(i + 2) % 7] += delayed_fish[i % 7];
        // Since fish take more than a week for first spawn, hold onto their spawn count in a vector
        // to use a week later
        delayed_fish[i % 7] = fish_memo[i % 7];
    }
    fish_count
}

pub fn part1(input: String) -> i64 {
    fish_count(input, 80)
}

pub fn part2(input: String) -> i64 {
    fish_count(input, 256)
}
