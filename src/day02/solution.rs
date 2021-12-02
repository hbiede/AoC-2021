pub fn part1(input: String) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    input
        .lines()
        .for_each(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            match split[0] {
                "up" => depth -= split[1].parse::<i32>().unwrap(),
                "down" => depth += split[1].parse::<i32>().unwrap(),
                "forward" => pos += split[1].parse::<i32>().unwrap(),
                _ => panic!("Unknown direction"),
            }
        });
    depth * pos
}

pub fn part2(input: String) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    input
        .lines()
        .for_each(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            match split[0] {
                "up" => aim -= split[1].parse::<i32>().unwrap(),
                "down" => aim += split[1].parse::<i32>().unwrap(),
                "forward" => {
                    let x = split[1].parse::<i32>().unwrap();
                    pos += x;
                    depth += aim * x;
                },
                _ => panic!("Unknown direction"),
            }
        });
    depth * pos
}
