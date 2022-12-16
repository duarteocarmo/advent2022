use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./6.txt") {
        for line in lines {
            let window_size = 14;
            if let Ok(inputs) = line {
                let mut chars: Vec<&str> = inputs.split("").collect();
                chars.retain(|&x| x.len() > 0);

                let chars_size = chars.len();

                for position in 1..=chars_size - window_size {
                    let candidate = &chars[position..position + window_size];
                    let uniques: Vec<_> = candidate.into_iter().unique().collect();

                    if candidate.len() == uniques.len() {
                        println!("Candidate: {:?}", candidate);
                        println!("Current index: {:?}", position);
                        let result = position + window_size;
                        println!("Answer: {:?}", result);
                        break;
                    }
                }
                // println!("{:?}", chars);
            };
        }
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
