use itertools::Itertools; // itertools = "0.8"
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_initial_vectors(line_to_process: String) -> HashMap<i32, Vec<String>> {
    let mut _stacks: HashMap<i32, Vec<String>> = HashMap::new();

    let mut thang: Vec<&str> = line_to_process.split(" ").collect();

    thang.retain(|&x| x.len() > 0);

    let b: Vec<i32> = thang.iter().flat_map(|s| s.parse().ok()).collect();

    for el in b.iter() {
        let vec = Vec::new();

        _stacks.entry(*el).or_insert(vec);
        // _stacks.insert(*el, vec);
    }

    _stacks
}

fn str_strip_numbers(s: &str) -> Vec<i64> {
    let re = Regex::new(r"\d+").unwrap();
    // iterate over all matches
    re.find_iter(s)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}

fn main() {
    // File hosts must exist in current path before this produces output
    //
    let mut lines_to_process = Vec::new();
    if let Ok(lines) = read_lines("./5.txt") {
        // Consumes the iterator, returns an (Optional) String
        //
        //
        //

        for line in lines {
            if let Ok(inputs) = line {
                lines_to_process.push(inputs);
            };
        }
    }

    let mut stacks: HashMap<i32, Vec<String>> = HashMap::new();

    for line in lines_to_process.clone() {
        if (!line.contains("[")) & (!line.contains("move")) & (line.len() > 3) {
            stacks = get_initial_vectors(line);
        }
    }

    // let moves = lines_to_process.into_iter().filter(|&i|i % 2 == 0).collect::<Vec<_>>();

    for line in lines_to_process.clone() {
        if line.contains("[") {
            let mut stack_number = 1;
            for position in (1..=200).step_by(4) {
                if position < line.len() {
                    // println!("{:?}", position);
                    let b: u8 = line.as_bytes()[position];
                    let c: char = b as char;
                    if c != ' ' {
                        match stacks.entry(stack_number) {
                            Entry::Vacant(_e) => {
                                // e.insert(vec![value]);
                            }
                            Entry::Occupied(mut e) => {
                                // e.get_mut().push(c.to_string());
                                e.get_mut().insert(0, c.to_string());
                            }
                        }
                    }
                    stack_number += 1
                }
            }
        }
    }

    for line in lines_to_process.clone() {
        if line.contains("move") {
            let numbers = str_strip_numbers(&line);
            let n_to_move = numbers[0] as i32;
            let origin = numbers[1];
            let destination = numbers[2];
            let mut moovers = Vec::new();

            // remove
            match stacks.entry(origin as i32) {
                Entry::Vacant(_e) => {
                    // e.insert(vec![value]);
                }
                Entry::Occupied(mut e) => {
                    let old_value = e.get_mut();
                    let current_size = old_value.len();
                    let new_size = current_size as i32 - n_to_move;

                    moovers = old_value[new_size as usize..].to_vec();

                    e.get_mut().truncate(new_size as usize);
                }
            }

            // println!("REMOVED!");
            // println!("Moovers: {:?}", moovers);

            // add
            match stacks.entry(destination as i32) {
                Entry::Vacant(_e) => {
                    // e.insert(vec![value]);
                }
                Entry::Occupied(mut e) => {
                    // moovers.reverse();
                    for to_push in moovers {
                        e.get_mut().push(to_push);
                    }
                }
            }
            // println!("ADDED!");
            // println!("STEP ENDED {:?}", stacks);
            // println!("========");
        }
    }

    // println!("FINAL {:?}", stacks);

    // for (key, value) in stacks.into_iter() {
    //     println!("{:?} / {:?}", key, value);
    // }
    //
    let mut final_code = Vec::new();

    for key in stacks.keys().sorted() {
        // println!("{:?} has {:?}", key, stacks[key]);
        final_code.push(stacks[key].last().clone().unwrap())
    }
    println!("ANSWER {:?}", final_code);
    // let clean_code = final_code.join("-");
    // println!("ANSWER {:?}", clean_code);
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
