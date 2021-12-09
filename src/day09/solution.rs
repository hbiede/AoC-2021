fn parse_locations(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}
fn find_low_spots(locations: Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut low_spots = vec![];
    for i in 0..locations.len() {
        for j in 0..locations[i].len() {
            // Continue if found to be adjacent to a lower spot
            if i > 0 && locations[i][j] >= locations[i - 1][j] {
                continue;
            }
            if i < locations.len() - 1 && locations[i][j] >= locations[i + 1][j] {
                continue;
            }
            if j > 0 && locations[i][j] >= locations[i][j - 1] {
                continue;
            }
            if j < locations[i].len() - 1 && locations[i][j] >= locations[i][j + 1] {
                continue;
            }
            // Local minimum found
            low_spots.push((i, j));
        }
    }
    low_spots
}

pub fn part1(input: String) -> i64 {
    let locations = parse_locations(input);
    find_low_spots(locations.clone())
        .iter()
        // Risk factor of a spot is equal to 1 plus the depth
        .map(|(i, j)| locations[*i][*j] as i64 + 1)
        .sum::<i64>()
}

pub fn part2(input: String) -> i64 {
    let locations = parse_locations(input);
    let mut basins = vec![];
    find_low_spots(locations.clone())
        .iter()
        .for_each(|spot| {
            // Perform a breadth-first search on all low spots to determine the size of the basin
            let mut queue = vec![*spot];
            // Track the map of visited spots to prevent double counting
            let mut in_basin = vec![
                vec![false; locations[0].len()];
                locations.len()
            ];
            while let Some(curr) = queue.pop() {
                in_basin[curr.0][curr.1] = true;
                // Check adjacent cells. If they're in the same basin, add them to the queue

                if curr.0 > 0 &&
                    // If not already visited, add to the queue
                    !in_basin[curr.0 - 1][curr.1] &&
                    // Not a basin barrier
                    locations[curr.0 - 1][curr.1] >= locations[curr.0][curr.1] &&
                    // 9's are barriers automatically
                    locations[curr.0 - 1][curr.1] != 9 {
                    queue.push((curr.0 - 1, curr.1));
                }
                if curr.0 < locations.len() - 1 &&
                    !in_basin[curr.0 + 1][curr.1] &&
                    locations[curr.0 + 1][curr.1] >= locations[curr.0][curr.1] &&
                    locations[curr.0 + 1][curr.1] != 9 {
                    queue.push((curr.0 + 1, curr.1));
                }
                if curr.1 > 0 &&
                    !in_basin[curr.0][curr.1 - 1] &&
                    locations[curr.0][curr.1 - 1] >= locations[curr.0][curr.1] &&
                    locations[curr.0][curr.1 - 1] != 9 {
                    queue.push((curr.0, curr.1 - 1));
                }
                if curr.1 < locations[curr.0].len() - 1 &&
                    !in_basin[curr.0][curr.1 + 1] &&
                    locations[curr.0][curr.1 + 1] >= locations[curr.0][curr.1] &&
                    locations[curr.0][curr.1 + 1] != 9 {
                    queue.push((curr.0, curr.1 + 1));
                }
            }
            basins.push(
                 in_basin.iter().fold(0, |acc, row| acc + row.iter().filter(|&&b| b).count() as i64) as i64
            );
        });
    // Sort descending by basin size
    basins.sort_by(|a, b| (*b).cmp(a));
    // Score is the product of the 3 largest basins
    basins[0..=2].iter().product()
}
