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

fn inspect_signal(cycle_count: i32, register_value: i32) -> i32 {
    let inspection_values: Vec<u32> = (20..=220).step_by(40).collect();

    if inspection_values.contains(&(cycle_count as u32)) {
        let result = cycle_count * register_value;
        println!(
            "AT CYCLE {:?}, REGISTER VALUE {:?} // RESULT {:?}",
            cycle_count, register_value, result
        );

        result
    } else {
        0
    }
}

fn part_1() {
    let lines = lines_from_file("./example.txt");

    let mut register_value = 1;
    let mut cycle_count = 0;
    let mut signal_strength = 0;

    for line in lines {
        if line == "noop" {
            // inspect_signal(cycle_count, register_value);
            cycle_count += 1;
            signal_strength += inspect_signal(cycle_count, register_value);
            continue;
        }

        if line.contains("addx") {
            let (_, count) = line.split_once(" ").unwrap();
            let count = count.parse::<i32>().unwrap();

            cycle_count += 1;
            signal_strength += inspect_signal(cycle_count, register_value);
            cycle_count += 1;
            signal_strength += inspect_signal(cycle_count, register_value);
            register_value += count;
            continue;
        }
    }
    // inspect_signal(cycle_count, register_value);
    //
    println!("RESULT {:?}", signal_strength)
}

fn main() {
    part_1();
    // inspect_signal(140, 3123);
}
