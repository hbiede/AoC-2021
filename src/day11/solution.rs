type Map = Vec<Vec<usize>>;
fn parse_octopuses(input: String) -> Map {
    input
        .lines()
        .map(|line| {
            line.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn process_single_step(map: Map) -> (Map, i32) {
    let mut new_map = map
        .iter()
        .map(|row| row.iter().map(|cell| cell + 1).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    // Fill queue with the coordinates of all elements == 9
    let mut queue = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &val)| {
                if val >= 9 {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(usize, usize)>>();
    let mut flashes = 0;
    while let Some(point) = queue.pop() {
        if new_map[point.1][point.0] == 0 {
            // This cell was added to the queue multiple times, so we don't need to process
            // it again
            continue;
        }
        new_map[point.1][point.0] = 0;
        flashes += 1;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_x = point.0 as i32 + i;
                let new_y = point.1 as i32 + j;
                if new_x < 0 || new_y < 0 {
                    // Out of bounds to the left or top
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if new_x >= new_map[0].len() || new_y >= new_map.len() {
                    // Out of bounds to the right or bottom
                    continue;
                }
                if new_map[new_y][new_x] == 0 {
                    // Already processed
                    continue;
                }
                new_map[new_y][new_x] += 1;
                if new_map[new_y][new_x] > 9 {
                    queue.push((new_x, new_y));
                }
            }
        }
    }
    (new_map, flashes)
}

pub fn part1(input: String) -> i64 {
    let mut map = parse_octopuses(input);
    let mut flashes = 0;
    for _ in 0..100 {
        let (new_map, new_flashes) = process_single_step(map.clone());
        map = new_map;
        flashes += new_flashes;
    }
    flashes as i64
}

pub fn part2(input: String) -> i64 {
    let mut map = parse_octopuses(input);
    let octopus_count = map.iter().map(|row| row.len() as i32).sum();
    let mut i = -1;
    loop {
        i += 1;
        let (new_map, new_flashes) = process_single_step(map.clone());
        map = new_map;
        if new_flashes == octopus_count {
            return i + 1;
        }
    }
}
