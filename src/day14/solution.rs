use std::collections::HashMap;

fn parse_insertions(input: String) -> HashMap<String, String> {
    let mut insertions = HashMap::new();
    input
        .lines()
        .for_each(|line| {
            let mut parts = line.split(" -> ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            insertions.insert(key, value);
        });
    insertions
}

fn find_polymer_score(input: String, iterations: usize) -> i64 {
    // Counts the number of symbols in the polymer
    let mut char_counts = HashMap::new();
    // Count the number of base pairs in the polymer
    let mut pair_counts = HashMap::new();

    let mut parts = input.split("\n\n");
    let chain = parts
        .next()
        .unwrap()
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let insertions = parse_insertions(parts.next().unwrap().to_string());
    insertions
        .values()
        .for_each(|c| {
            // Initialize the counts for the characters in the insertion
            char_counts.insert(c.to_string(), 0);
        });
    chain
        .iter()
        .for_each(|c| {
            // Count the number of times each character appears in the chain
            char_counts.entry(c.to_string()).and_modify(|count| *count += 1);
        });
    chain
        .windows(2)
        .for_each(|c| {
            // Fill with initial pairs
            pair_counts
                .entry((c[0], c[1]))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
    for _ in 0..iterations {
        let mut new_pair_counts = pair_counts.clone();
        for &(a, b) in pair_counts.keys() {
            let pair_count = *(pair_counts.get(&(a, b)).unwrap());
            if pair_count < 1 {
                // This pair is not present in the chain currently, skip it
                continue;
            }

            // Get the polymer symbol to insert
            let insert = insertions
                .get(&*format!("{}{}", a, b))
                .unwrap()
                .chars()
                .next()
                .unwrap();
            // Add new insert to overall char count
            char_counts
                .entry(insert.to_string())
                .and_modify(|count| *count += pair_count)
                .or_insert(pair_count);
            // Remove initial pair from pair count
            new_pair_counts
                .entry((a, b))
                .and_modify(|count| *count -= pair_count);
            // Insert front new pair into pair count
            new_pair_counts
                .entry((a, insert))
                .and_modify(|count| *count += pair_count)
                .or_insert(pair_count);
            // Insert back new pair into pair count
            new_pair_counts
                .entry((insert, b))
                .and_modify(|count| *count += pair_count)
                .or_insert(pair_count);
        }
        pair_counts = new_pair_counts;
    }
    char_counts.values().max().unwrap() - (
        *(char_counts
            .values()
            // Don't count zeroes
            .filter(|v| ((**v) as i64).is_positive())
            .min()
            .unwrap())
    ) as i64
}

pub fn part1(input: String) -> i64 {
    find_polymer_score(input, 10)
}

pub fn part2(input: String) -> i64 {
    find_polymer_score(input, 40)
}
