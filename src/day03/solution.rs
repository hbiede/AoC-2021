fn most_common_bit(lines: Vec<&str>, invert: bool) -> Vec<i32> {
    let mut bits = vec![0; lines.clone()[0].chars().count()];
    lines
        .clone()
        .iter()
        .for_each(|line| {
            line.chars()
                .enumerate()
                .for_each(|(i, c)| {
                    if c == '1' {
                        bits[i] += 1;
                    }
                })
        });
    bits.iter()
        .map(|bit| {
            if (!invert) ^ (*bit >= (lines.clone().len() as i32 - *bit)) {
                0
            } else {
                1
            }
        }).collect()
}

fn find_lone_value(bits: Vec<&str>, invert: bool) -> i32 {
    let mut remaining_bits = bits.clone();
    let mut most_common = most_common_bit(bits.clone(), invert);
    let mut index: usize = 0;
    while remaining_bits.len() > 1 {
        // Filter remaining bits so only the strings with the most common bit at the
        // current index remain
        remaining_bits = remaining_bits.iter()
            .filter(|bit| {
                (**bit).to_string().chars().nth(index).unwrap()
                    == format!("{}", most_common[index]).chars().next().unwrap()
            })
            .copied()
            .collect::<Vec<&str>>();
        // Update most common bit list
        most_common = most_common_bit(remaining_bits.clone(), invert);
        index += 1;
    }
    remaining_bits[0]
        .chars()
        .fold(0, |acc, bit| acc * 2 + (bit as i32 - '0' as i32))
}

pub fn part1(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let most_common_bit = most_common_bit(lines.clone(), false);
    // Gamma is the decimal value of the most common bits
    let gamma = most_common_bit.iter().fold(0, |acc, bit| acc * 2 + bit);
    // Epsilon is the binary inversion of gamma
    let epsilon = !gamma & ((1 << most_common_bit.len()) - 1);
    (gamma * epsilon) as i64
}

pub fn part2(input: String) -> i64 {
    let bits = input.lines().collect::<Vec<&str>>();
    (find_lone_value(bits.clone(), false)
        * find_lone_value(bits.clone(), true)) as i64
}
