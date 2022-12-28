use std::collections::HashSet;

use std::{env, fs};

fn read_file_input(file: String) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("./").join(file);

    fs::read_to_string(filepath).unwrap()
}

fn main() {
    let input = read_file_input("example.txt".to_string());

    println!("{}", calculate(&input, 1));
    println!("{}", calculate(&input, 9));
}

fn calculate(input: &String, length: usize) -> usize {
    let mut rope = vec![(0i32, 0i32); length + 1];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in input.lines() {
        let instruction = line.split_once(' ').unwrap();

        let (dx, dy) = match instruction.0 {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => unreachable!(),
        };

        let steps = instruction.1.parse::<usize>().unwrap();

        for _ in 0..steps {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..rope.len() {
                let (dx, dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i].0 += dx.signum();
                    rope[i].1 += dy.signum();
                }
            }
            visited.insert(rope[length]);
        }
    }

    return visited.len();
}
