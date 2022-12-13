use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn process_puzzle(puzzle_line: String) -> i32 {
    //   println!("The puzzle is : {puzzle_line}");
    let strings = puzzle_line.split(" ");
    let vec: Vec<&str> = strings.collect();
    let oponent_play = vec[0];
    let my_play = vec[1];

    let mut total_score = 0;

    if my_play == "X" {
        total_score += 1;

        match oponent_play {
            "A" => total_score += 3, //rock vs rock
            "B" => total_score += 0, // rock vs pa[er
            "C" => total_score += 6, // rock vs scicores
            _ => println!("ERROR"),
        }
    } else if my_play == "Y" {
        total_score += 2;

        match oponent_play {
            "A" => total_score += 6,
            "B" => total_score += 3,
            "C" => total_score += 0,
            _ => println!("ERROR"),
        }
    } else {
        total_score += 3;
        match oponent_play {
            "A" => total_score += 0,
            "B" => total_score += 6,
            "C" => total_score += 3,
            _ => println!("ERROR"),
        }
    };

    //    println!("Total score: {total_score}");

    total_score
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./2.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut final_score = 0;
        for line in lines {
            if let Ok(inputs) = line {
                println!("Inputs: {}", inputs);
                let score = process_puzzle(inputs);
                final_score += score;
                println!("Score - {score}")
            }
        }
        println!("Final score {final_score}")
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
