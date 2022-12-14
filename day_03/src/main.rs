use array_tool::vec::Intersect;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// fn process_puzzle(puzzle_line: String) -> i32 {
//     let strings = puzzle_line.split("");
//     let vec: Vec<&str> = strings.filter(|s| !s.is_empty()).collect();
//     let vec_size = vec.len();
//     let half_index = vec_size / 2;
//     let first_half = vec[..half_index].to_vec();
//     let second_half = vec[half_index..].to_vec();
//     let common_elements = first_half.intersect(second_half);

//     // println!("{:?}", vec);
//     // println!("{:?}", common_elements);

//     let shared_item: String = common_elements[0].to_string();
//     let lowercase = "abcdefghijklmnopqrstuvwxyz";
//     let uppercase = lowercase.to_uppercase();
//     let result;
//     // let shared_item_position = lowercase.find(&shared_item).unwrap_or(0);

//     if lowercase.contains(&shared_item) {
//         result = lowercase.find(&shared_item).unwrap_or(0) + 1;
//     } else if uppercase.contains(&shared_item) {
//         result = uppercase.find(&shared_item).unwrap_or(0) + 27;
//     } else {
//         result = 0;
//     };

//     result as i32
// }
//
fn process_group(list_of_runsacks: Vec<String>) -> i32 {
    println!("{:?}", list_of_runsacks);
    let first_element = list_of_runsacks[0].chars().collect::<Vec<_>>();
    let second_element = list_of_runsacks[1].chars().collect::<Vec<_>>();
    let third_element = list_of_runsacks[2].chars().collect::<Vec<_>>();

    // let second_element = list_of_runsacks[1];
    // let third_element = list_of_runsacks[2];
    let first_element = first_element.intersect(second_element);
    let common_elements = first_element.intersect(third_element);

    println!("{:?}", common_elements);
    let shared_item: String = common_elements[0].to_string();
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = lowercase.to_uppercase();
    let result;
    // let shared_item_position = lowercase.find(&shared_item).unwrap_or(0);

    if lowercase.contains(&shared_item) {
        result = lowercase.find(&shared_item).unwrap_or(0) + 1;
    } else if uppercase.contains(&shared_item) {
        result = uppercase.find(&shared_item).unwrap_or(0) + 27;
    } else {
        result = 0;
    };

    result as i32
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./3.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut final_score = 0;
        let mut line_counter = 0;
        let mut lines_to_process = Vec::new();
        for line in lines {
            if let Ok(inputs) = line {
                lines_to_process.push(inputs.to_string());
                line_counter += 1;

                if line_counter == 3 {
                    // println!("{:?}", lines_to_process);
                    let score = process_group(lines_to_process);
                    final_score += score;
                    println!("{:?}", score);
                    line_counter = 0;
                    lines_to_process = Vec::new();
                }

                // println!("Inputs: {}", inputs);
                // let points = process_puzzle(inputs);
                // println!("POints: {}", points);
                // final_score += points;
            };
        }

        println!("FINAL : {:?}", final_score)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
