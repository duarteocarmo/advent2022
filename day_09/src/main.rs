use hashbrown::HashSet;
use nalgebra::distance;
use nalgebra::point;
use nalgebra::geometry::Point2;
use nalgebra::vector;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn part_1() {
    let lines = lines_from_file("./example.txt");
    let mut head_position = point!(0.0, 0.0);
    let mut tail_position = point!(0.0, 0.0);

    for (step, line) in lines.iter().enumerate() {
        let (direction, count) = line.split_once(" ").unwrap();
        let count = count.parse::<i32>().unwrap();

        let out = match direction {
            "R" => (1.0, 0.0),
            "L" => (-1.0, 0.0),
            "U" => (0.0, 1.0),
            "D" => (0.0, -1.0),
            _ => panic!("SHIT HIT THE FAN"),
        };

        // let direction = vec2(out.0, out.1);
        //
        let direction = vector![out.0, out.1];

        println!("========START STEP {:?}=======", step + 1);
        // println!("HEAD AT: {:?}", head_position);
        // println!("TAIL AT: {:?}", tail_position);


        let mut switch_next_step = false;
        let mut fixer = point!(0.0, 0.0);

        for mini_step in 0..count {
            head_position += direction;
            println!("HEAD TO: {:?}", head_position);
            if (mini_step == 0) & (step == 0) {
                println!("~~");
                continue;
            }

            let diagonal =
                (head_position.x != tail_position.x) & (head_position.y != tail_position.y);

            if switch_next_step == true {
                tail_position = fixer;
                switch_next_step = false;
            } else if diagonal == true {
                println!("DIAGONAL");
                switch_next_step = true;
                fixer = head_position;
                // fixer = head_position;
            } else {
                tail_position += direction;
            }

            println!("TAIL TO: {:?}", tail_position);
            println!("~~")
            // println!("STEP {:?}, HEAD {:?}", mini_step, head_position);
        }

        println!("===== END STEP =====")
    }
}

fn part_2() {}



fn main() {
    println!("{:?}", part_1());
    // println!("{:?}", part_2());
}


use std::vec;

use hashbrown::HashSet;

use crate::{problem, Solution};

type Point = aoc_lib::Point<i32>;

pub struct Day09;

impl Solution for Day09 {
    fn name(&self) -> &'static str {
        "Rope Bridge"
    }

    fn part_a(&self) -> String {
        let raw = problem::load(2022, 9);
        process(&raw, 1).to_string()
    }

    fn part_b(&self) -> String {
        let raw = problem::load(2022, 9);
        process(&raw, 9).to_string()
    }
}

fn process(raw: &str, count: usize) -> usize {
    let orders = raw.lines().map(from_line).collect::<Vec<_>>();
    let mut tail_pios = HashSet::new();
    let mut knots = vec![Point::new(0, 0); count + 1];

    tail_pios.insert(*knots.last().unwrap());
    for (dir, count) in orders {
        for _ in 0..count {
            knots[0] += dir;

            for i in 1..knots.len() {
                let diff = knots[i - 1] - knots[i];
                if diff.abs().max_value() <= 1 {
                    continue;
                }

                knots[i] += diff.normalize();
            }
            tail_pios.insert(*knots.last().unwrap());
        }
    }

    tail_pios.len()
}

// Direction, count
fn from_line(imp: &str) -> (Point, u32) {
    let (direction, count) = imp.split_once(' ').unwrap();
    let count = count.parse::<i32>().unwrap();

    let out = match direction {
        "R" => Point::new(1, 0),
        "L" => Point::new(-1, 0),
        "U" => Point::new(0, -1),
        "D" => Point::new(0, 1),
        _ => panic!("Invalid direction"),
    };

    (out, count as u32)
}
