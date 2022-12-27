use std::{
    cmp,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn part_1() -> i32 {
    let lines = lines_from_file("./8.txt");

    let mut elements: Vec<Vec<i32>> = Vec::new();
    let mut visible: Vec<(i32, i32)> = Vec::new();
    let total_lines = lines.len();
    let row_size = lines[0].len();
    let mut total_visible = row_size * 2 + total_lines * 2 - 4;

    for (index, line) in lines.iter().enumerate() {
        let row: Vec<i32> = line.split("").filter_map(|s| s.parse().ok()).collect();
        elements.push(row.clone());

        if (index == 0) | (index == total_lines - 1) {
            continue;
        }

        for element in 1..row.len() - 1 {
            let value = row[element];

            let max_right = row[element + 1..].iter().max().copied().unwrap();

            let max_left = row[..element].iter().max().copied().unwrap();

            if value > max_left {
                visible.push((index.try_into().unwrap(), element.try_into().unwrap()));
                continue;
            }
            if value > max_right {
                visible.push((index.try_into().unwrap(), element.try_into().unwrap()));
                continue;
            }
        }
    }

    for col_number in 1..row_size - 1 {
        let mut col = Vec::new();

        for element in elements.iter() {
            col.push(element[col_number])
        }
        for element in 1..col.len() - 1 {
            let value = col[element];

            let max_bottom = col[element + 1..].iter().max().copied().unwrap();

            let max_top = col[..element].iter().max().copied().unwrap();

            if value > max_bottom {
                visible.push((element.try_into().unwrap(), col_number.try_into().unwrap()));
                continue;
            }
            if value > max_top {
                visible.push((element.try_into().unwrap(), col_number.try_into().unwrap()));
                continue;
            }
        }
    }

    let mut unique_visible: Vec<(i32, i32)> = Vec::new();

    for element in visible {
        if unique_visible.contains(&element) {
            continue;
        }

        unique_visible.push(element);
    }

    total_visible += unique_visible.len();

    total_visible.try_into().unwrap()
}

fn part_2() -> i32 {
    let lines = lines_from_file("./8.txt");

    let mut elements: Vec<Vec<i32>> = Vec::new();
    let row_size = lines[0].len();
    let mut left_right_scores: Vec<i32> = Vec::new();
    let mut lr_positions: Vec<(i32, i32)> = Vec::new();
    let mut top_bottom_scores: Vec<i32> = Vec::new();
    let mut tb_positions: Vec<(i32, i32)> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let row: Vec<i32> = line.split("").filter_map(|s| s.parse().ok()).collect();
        elements.push(row.clone());

        // if (index == 0) | (index == total_lines - 1) {
        //     continue;
        // }

        for element in 0..row.len() {
            let value = row[element];

            let view_right = &row[element + 1..];
            let mut right_score = 0;

            for r_e in view_right {
                if &value > r_e {
                    right_score += 1;
                } else {
                    right_score += 1;
                    break;
                }
            }

            let view_left = &row[..element];
            let mut left_score = 0;

            for l_e in view_left.iter().rev() {
                if &value > l_e {
                    left_score += 1;
                } else {
                    left_score += 1;
                    break;
                }
            }

            let left_right_score = right_score * left_score;
            lr_positions.push((index.try_into().unwrap(), element.try_into().unwrap()));
            left_right_scores.push(left_right_score);
        }
    }

    for col_number in 0..row_size {
        let mut col = Vec::new();

        for element in elements.iter() {
            col.push(element[col_number])
        }

        for element in 0..col.len() {
            let value = col[element];

            let view_bottom = &col[element + 1..];
            let mut bottom_score = 0;

            for b_e in view_bottom {
                if &value > b_e {
                    bottom_score += 1;
                } else {
                    bottom_score += 1;
                    break;
                }
            }

            let view_top = &col[..element];
            let mut top_score = 0;

            for t_e in view_top.iter().rev() {
                if &value > t_e {
                    top_score += 1;
                } else {
                    top_score += 1;
                    break;
                }
            }

            let top_bottom_score = bottom_score * top_score;
            tb_positions.push((element.try_into().unwrap(), col_number.try_into().unwrap()));
            top_bottom_scores.push(top_bottom_score);
        }
    }

    let mut max_scenic_score = 0;

    for (index, element) in lr_positions.iter().enumerate() {
        let lr_score = left_right_scores[index];
        let tb_position = tb_positions.iter().position(|&x| x == *element).unwrap();
        let tb_score = top_bottom_scores[tb_position];

        let scenic_score = lr_score * tb_score;

        max_scenic_score = cmp::max(scenic_score, max_scenic_score);
    }

    max_scenic_score
}

fn main() {
    println!("{:?}", part_1());
    println!("{:?}", part_2());
}
