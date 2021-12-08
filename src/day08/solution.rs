use std::collections::HashSet;

pub fn part1(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let output = line.split(" | ").nth(1).unwrap();
            output.split(' ').filter(|word| {
                let len = word.trim().len();
                len == 2 || len == 3 || len == 4 || len == 7
            }).count() as i64
        })
        .sum::<i64>()
}

fn find_digit_strings(line: String) -> Vec<String> {
    let mut connections = HashSet::new();
    let words = line.split(" | ").next().unwrap().split(' ').collect::<Vec<&str>>();
    for word in words {
        let mut word = word.trim().chars().collect::<Vec<char>>();
        // Sort to allow for easy comparison later
        word.sort_unstable();
        connections.insert(word);
    }
    let mut nums = vec![vec![' '; 0]; 10];
    for word in connections.clone() {
        // Digits with unique segment counts
        if word.len() == 2 {
            nums[1] = word;
        } else if word.len() == 3 {
            nums[7] = word;
        } else if word.len() == 4 {
            nums[4] = word;
        } else if word.len() == 7 {
            nums[8] = word;
        }
    }
    for connection in connections.clone() {
        if connection.len() == 5 && nums[3].is_empty() {
            // Find 3
            let mut seven = nums[7].clone();
            connection.iter().for_each(|c| {
                let pos = seven.iter().position(|&x| x == *c);
                if let Some(pos) = pos {
                    seven.remove(pos);
                }
            });
            if seven.is_empty() {
                // 3 is the only digit that uses exactly 5 segments and entirely overlaps with 7
                nums[3] = connection;
            }
        } else if connection.len() == 6 && nums[6].is_empty() {
            // Find 6
            let mut seven = nums[7].clone();
            connection.iter().for_each(|c| {
                let pos = seven.iter().position(|&x| x == *c);
                if let Some(pos) = pos {
                    seven.remove(pos);
                }
            });
            if seven.len() == 1 {
                // 6 is the only digit that uses exactly 6 segments and only leaves one unused
                // segment from 7
                nums[6] = connection;
            }
        }
    }
    for connection in connections.clone() {
        if connection.len() == 5 && !connection.iter().zip(nums[3].clone()).all(|(a, b)| *a == b) {
            // Find 2 and 5
            if nums[2].is_empty() && nums[5].is_empty() {
                let mut six = nums[6].clone();
                connection.iter().for_each(|c| {
                    let pos = six.iter().position(|&x| x == *c);
                    if let Some(pos) = pos {
                        six.remove(pos);
                    }
                });
                if six.len() == 1 {
                    // 5 leaves one unused segment from 6
                    nums[5] = connection;
                } else {
                    // 2 leaves two unused segments from 6
                    nums[2] = connection;
                }
            } else if nums[2].is_empty() {
                // 3 and 5 are already filled, must be 2
                nums[2] = connection;
            } else if nums[5].is_empty() {
                // 2 and 3 are already filled, must be 5
                nums[5] = connection;
            }
        }
    }
    for connection in connections {
        if connection.len() == 6 && !connection.iter().zip(nums[6].clone()).all(|(a, b)| *a == b) {
            // Find 9 and 0
            if nums[9].is_empty() && nums[0].is_empty() {
                let mut word_clone = connection.clone();
                nums[5].clone().iter().for_each(|c| {
                    let pos = word_clone.iter().position(|&x| x == *c);
                    if let Some(pos) = pos {
                        word_clone.remove(pos);
                    }
                });
                if word_clone.len() == 1 {
                    // 5 leaves one unused segment from 9
                    nums[9] = connection;
                } else {
                    // 5 leaves two unused segments from 0
                    nums[0] = connection;
                }
            } else if nums[0].is_empty() {
                // 6 and 9 are already filled, must be 0
                nums[0] = connection;
            } else if nums[9].is_empty() {
                // 6 and 0 are already filled, must be 9
                nums[9] = connection;
            }
        }
    }
    nums.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>()
}

pub fn part2(input: String) -> i64 {
    let outputs = input
        .lines()
        .map(|line| {
            let mut split = (*line).split(" | ");
            let connections = split.next().unwrap();
            let output = split.next().unwrap();
            let num_strings = find_digit_strings(connections.to_string());

            output
                .split(' ')
                .into_iter()
                .map(|digit| {
                    num_strings
                        .iter()
                        // Find the digit string that matches the digit in question
                        .position(|x| {
                            let mut digit_chars = digit.chars().collect::<Vec<char>>();
                            digit_chars.sort_unstable();
                            let sorted_digit = digit_chars.iter().collect::<String>();
                            x.eq(&sorted_digit)
                        })
                        .unwrap()
                })
                .fold(0, |acc, x| acc * 10 + x)
        });
    outputs.sum::<usize>() as i64
}
