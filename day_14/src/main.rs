use nalgebra::geometry::Point2;
use nalgebra::point;
use nalgebra::DMatrix;
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

fn main() {
    let lines = lines_from_file("./example.txt");

    for line in lines {
        println!("{:?}", line);
        let mut points: Vec<Point2<usize>> = Vec::new();
        let mut map = DMatrix::<f32>::zeros(600, 600);

        let points_strings = line.split(" -> ").collect::<Vec<&str>>();
        for point in points_strings {
            let x = point.split_once(",").unwrap().0.parse::<usize>().unwrap();
            let y = point.split_once(",").unwrap().1.parse::<usize>().unwrap();
            let p = point![x, y];
            points.push(p);
        }

        println!("{:?}", points);
        for p in points.windows(2) {
            println!("{:?}", p[0]);
            println!("{:?}", p[1]);

            if p[0].x == p[1].x {
                println!("Horizontal");

                let mut slice = map.slice_mut((p[0].x, p[0].y), (1, 1));
                slice.fill(1.0);

                // slice = 1.0_f32;
            } else if p[0].y == p[1].y {
                println!("Vertical");
            }
        }

        println!("{:?}", map);
    }

    // .unwrap()
    // .to_string()
}
