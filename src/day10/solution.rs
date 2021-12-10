fn find_syntax_error_score(line: &str) -> i32 {
    let mut stack = vec![' '; 0];
    for char in line.chars() {
        match char {
            '[' => stack.push('['),
            '{' => stack.push('{'),
            '(' => stack.push('('),
            '<' => stack.push('<'),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return 3;
                }
            },
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return 57;
                }
            },
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return 1197;
                }
            },
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return 25137;
                }
            },
            _ => {}
        }
    }
    if stack.is_empty() {
        return 0;
    }
    1
}

fn find_auto_complete_score(line: &str) -> i64 {
    let mut stack = vec![' '; 0];
    for char in line.chars() {
        match char {
            '(' => stack.push('('),
            '[' => stack.push('['),
            '{' => stack.push('{'),
            '<' => stack.push('<'),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return -1;
                }
            },
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return -1;
                }
            },
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return -1;
                }
            },
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return -1;
                }
            },
            _ => {}
        }
    }
    if stack.is_empty() {
        return 0;
    }
    let mut auto_correct_score = 0i64;
    while let Some(char) = stack.pop() {
        match char {
            '(' => {
                auto_correct_score = auto_correct_score * 5 + 1;
            },
            '[' => {
                auto_correct_score = auto_correct_score * 5 + 2;
            },
            '{' => {
                auto_correct_score = auto_correct_score * 5 + 3;
            },
            '<' => {
                auto_correct_score = auto_correct_score * 5 + 4;
            },
            _ => {},
        }
    }
    auto_correct_score
}

pub fn part1(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            find_syntax_error_score(line)
        })
        .filter(|&score| score >= 0)
        .sum::<i32>() as i64
}

pub fn part2(input: String) -> i64 {
    let mut auto_complete_scores = input
        .lines()
        .map(|line| {
            find_auto_complete_score(line)
        })
        .filter(|&score| score > 0)
        .collect::<Vec<i64>>();
    auto_complete_scores.sort_unstable();
    auto_complete_scores[(auto_complete_scores.len() as f32 / 2f32).floor() as usize]
}
