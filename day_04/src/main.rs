use array_tool::vec::Intersect;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn process_group(line_to_process: String) -> bool {
    println!("Spec: {:?}", line_to_process);
    let line_elements: Vec<&str> = line_to_process.split(",").collect();
    let first_range: Vec<i32> = line_elements[0]
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();
    let second_range: Vec<i32> = line_elements[1]
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();

    let first_vec: Vec<i32> = (first_range[0]..first_range[1] + 1).collect();
    let second_vec: Vec<i32> = (second_range[0]..second_range[1] + 1).collect();

    let intersection = first_vec.intersect(second_vec.clone());
    let intersection_size = intersection.len();

    if intersection_size > 0 {
        return true;
    } else {
        return false;
    }

    // if intersection == false {
    //     intersection = second_vec.iter().all(|item| first_vec.contains(item));
    // }

    // println!("{:?}", first_vec);
    // println!("{:?}", second_vec);
    // println!("{:?}", intersection);
    // println!("{:?}", intersection_size);
    // println!("=======");

    // true
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./4.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut total_score = 0;
        for line in lines {
            if let Ok(inputs) = line {
                let has_intersection = process_group(inputs);
                if has_intersection == true {
                    total_score += 1;
                }
            };
        }
        println!("Final score: {:?}", total_score);
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
