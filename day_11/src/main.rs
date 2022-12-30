use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
struct Monkey {
    id: i32,
    starting_items: Vec<i32>,
    test: i32,
    throw_true: i32,
    operation: String,
    operation_arg: i32,
    throw_false: i32,
}


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1() {
    let lines = lines_from_file("./example.txt");

    let mut m = Monkey::default();

    for line in lines {
        if line.contains("Monkey") {
            let id = line
                .split_once(" ")
                .unwrap()
                .1
                .replace(":", "")
                .parse::<i32>()
                .unwrap();

            // println!("Monkey {:?}", id);
            m.id = id;
        } else if line.contains("Starting items") {
            let starting_items_strings: _ = line
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .collect::<Vec<&str>>();

            let starting_items: Vec<i32> = starting_items_strings
                .iter()
                .flat_map(|s| s.parse().ok())
                .collect();

            m.starting_items = starting_items;

            // println!("Starting items: {:?}", starting_items);
        } else if line.contains("Operation") {
            

        } else if line.contains("Test:") {
            let test = line
                .split_once("divisible by")
                .unwrap()
                .1
                .trim()
                .parse::<i32>()
                .unwrap();
            // println!("Divisible test: {:?}", test);
            m.test = test;
        } else if line.contains("If true") {
            let throw_true = line
                .split_once("to monkey")
                .unwrap()
                .1
                .trim()
                .parse::<i32>()
                .unwrap();

            // println!("IF TRUE: {:?}", throw_true);
            m.throw_true = throw_true;
        } else if line.contains("If false") {
            let throw_false = line
                .split_once("to monkey")
                .unwrap()
                .1
                .trim()
                .parse::<i32>()
                .unwrap();

            // println!("IF FALSE: {:?}", throw_false);
            m.throw_false = throw_false;
        } else {
            println!("{:?}", m);
        }
    }
}

fn main() {
    part_1();
}
