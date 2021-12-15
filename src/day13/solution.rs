use std::collections::HashSet;

type Page = HashSet<(i32, i32)>;

fn get_coord_finder(fold_instructions: String) -> Box<dyn Fn(i32, i32) -> (i32, i32)> {
    let mut x_folds = vec![0; 0];
    let mut y_folds = vec![0; 0];
    fold_instructions
        .lines()
        .for_each(|line| {
            let mut fold = line.split(' ').last().unwrap().split('=');
            let dir = fold.next().unwrap();
            let coord = fold.next().unwrap().parse::<i32>().unwrap();
            if dir == "x" {
                x_folds.push(coord);
            } else {
                y_folds.push(coord);
            }
        });
    Box::new(move |x, y| {
        let mut x_fold: i32 = x;
        let mut y_fold: i32 = y;
        for fold in x_folds.iter() {
            if x_fold > *fold {
                x_fold = 2 * fold - x_fold;
            }
        }
        for fold in y_folds.iter() {
            if y_fold > *fold {
                y_fold = 2 * fold - y_fold;
            }
        }
        (x_fold, y_fold)
    })
}

fn get_folded_page(input: String, first_fold_only: bool) -> (Page, (i32, i32)) {
    let mut page: Page = HashSet::new();
    let mut instruction_parts = input.split("\n\n");
    let coords = instruction_parts.next().unwrap();
    let coord_finder = get_coord_finder(if first_fold_only {
        instruction_parts.next().unwrap().lines().next().unwrap().to_string()
    } else {
        instruction_parts.next().unwrap().to_string()
    });
    let mut x_max = 0;
    let mut y_max = 0;
    coords
        .lines()
        .for_each(|line| {
            let mut coord_parts = line.split(',');
            let x = coord_parts.next().unwrap().parse::<i32>().unwrap();
            let y = coord_parts.next().unwrap().parse::<i32>().unwrap();
            let (x_fold, y_fold) = coord_finder(x, y);
            if x_fold > x_max {
                x_max = x_fold;
            }
            if y_fold > y_max {
                y_max = y_fold;
            }
            page.insert((x_fold, y_fold));
        });
    (page, (x_max, y_max))
}

pub fn part1(input: String) -> i64 {
    get_folded_page(input, true).0.len() as i64
}

pub fn part2(input: String) -> i64 {
    let result = get_folded_page(input, false);

    // I'll just let the user read it on paper
    for y in 0..=result.1.1 {
        for x in 0..=result.1.0 {
            if result.0.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    result.0.len() as i64
}
