use nalgebra::geometry::Point2;
use nalgebra::point;
use std::collections::VecDeque;
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

#[derive(Debug, Clone)]
struct HeightMap {
    data: Vec<Vec<i32>>,
    current: Point2<i32>,
    start: Point2<i32>,
    end: Point2<i32>,
}

fn parse_puzze(input: &Vec<String>) -> HeightMap {
    let mut out: Vec<Vec<i32>> = Vec::new();
    let mut start = point![0, 0];
    let mut end = point![0, 0];

    for line in input {
        let mut row: Vec<i32> = Vec::new();

        for j in line.chars() {
            match j {
                'S' => {
                    row.push(0);
                    start = point![row.len() as i32 - 1, out.len() as i32];
                }
                'E' => {
                    row.push(25);
                    end = point![row.len() as i32 - 1, out.len() as i32];
                }
                _ => row.push(j as i32 - 97),
            }
        }

        out.push(row);
    }

    HeightMap {
        data: out,
        current: start,
        start,
        end,
    }
}

fn solve_map(
    map: &HeightMap,
    allow: fn(i32, i32) -> bool,
    solve: impl Fn(Point2<i32>) -> bool,
) -> Option<i32> {
    let mut visited = vec![vec![false; map.data[0].len()]; map.data.len()];
    println!("Visited: {:?}", visited);
    let mut queue: VecDeque<(Point2<i32>, Vec<i32>)> = VecDeque::new();
    queue.push_back((map.current, Vec::new()));

    while !queue.is_empty() {
        let (current, history) = queue.pop_front().unwrap();
        if solve(current) {
            let lenght = history.len() as i32;
            return Some(lenght);
        }

        let current_height = map.data[current.y as usize][current.x as usize];

        let mut check_neighbour = |x: usize, y: usize| {
            if x >= map.data[0].len()
                || y >= map.data.len()
                || visited[y][x]
                || !allow(map.data[y][x], current_height)
            {
                return;
            }

            visited[y][x] = true;
            let mut new_history = history.clone();
            new_history.push(current);
            queue.push_back((point![x as i32, y as i32], new_history));
        };

        check_neighbour(current.x + 1, current.y);
        check_neighbour(current.x, current.y + 1);
        check_neighbour(current.x.wrapping_sub(1), current.y);
        check_neighbour(current.x, current.y.wrapping_sub(1));
    }

    Some(3)
}

fn main() {
    let lines = lines_from_file("./example.txt");
    let map = parse_puzze(&lines);
    println!("{:?}", map);

    solve_map(&map, |a, b| a <= b + 1, |c| c == map.end);
    // .unwrap()
    // .to_string()
}
