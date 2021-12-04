type Board = Vec<Vec<i32>>;
fn gen_board(input: String) -> Board {
    input
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .map(|number| number.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
            })
        .collect::<Vec<Vec<i32>>>()
}

fn parse_input(input: String) -> (Vec<i32>, Vec<Board>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let numbers = lines[0]
        .split(',')
        .map(|number| number.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let boards = lines[2..]
        .chunks(6)
        .map(|board| {
            gen_board(board[0..5].join("\n"))
        })
        .collect::<Vec<Board>>();
    (numbers, boards)
}

fn has_won(board: Vec<Vec<bool>>) -> bool {
    let mut has_won = false;
    for i in 0..board.len() {
        let mut row = true;
        let mut col = true;
        for j in 0..board.len() {
            row = row && board[i][j];
            col = col && board[j][i];
        }
        if row || col {
            has_won = true;
        }
    }
    has_won
}

fn get_turns_and_scores(numbers: Vec<i32>, board: Board) -> (i32, i32) {
    let mut played = board
        .iter()
        .map(|row| {
            row.iter().map(|_| false).collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    // Find the first number in the board
    let mut index = 0;
    'game_loop: while index < numbers.len() {
        if has_won(played.clone()) {
            break;
        } else if index == numbers.len() - 1 {
            return ((numbers.len() + 1) as i32, 0);
        }
        for row_index in 0..board.len() {
            for square_index in 0..board[row_index].len() {
                if board[row_index][square_index] == numbers[index] {
                    played[row_index][square_index] = true;
                    index += 1;
                    continue 'game_loop;
                }
            }
        }
        // Number not found in the board
        index += 1;
    }
    let score =
        // Sum of unmarked squares
        played
            .iter()
            .enumerate()
            .fold(0, |acc, (row_index, row)| {
                acc + row.iter().enumerate().filter(|(_, square)| !(**square)).fold(0, |row_acc, (square_index, _)| row_acc + board[row_index][square_index])
            })
        // Times the last played number
        * numbers[index - 1];
    (index as i32 - 1, score)
}

pub fn part1(input: String) -> i32 {
    let (numbers, boards) = parse_input(input);
    let mut solutions: Vec<(i32, i32)> = vec![];
    let min = numbers.len();
    for i in 0..boards.len() {
        // Limit to the minimum number of turns to win as a short circuit
        let (turns, score) = get_turns_and_scores(numbers[0..min].to_vec(), boards[i].clone());
        solutions.push((turns, score));
    }
    // Find minimum turns in solutions and return the score
    solutions.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().1
}

pub fn part2(input: String) -> i32 {
    let (numbers, boards) = parse_input(input);
    let mut solutions: Vec<(i32, i32)> = vec![];
    for i in 0..boards.len() {
        let (turns, score) = get_turns_and_scores(numbers.clone(), boards[i].clone());
        solutions.push((turns, score));
    }
    // Find minimum turns in solutions and return the score
    solutions.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().1
}
