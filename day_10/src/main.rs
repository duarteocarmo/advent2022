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

fn draw_sprite(position: i32, cycle_count: i32) {
    let positions = vec![position - 1, position, position + 1];
    let mut zero_vec = vec![0; 40];

    for p in positions {
        if p >= 0 {
            zero_vec[p as usize] = 1;
        }
    }

    println!(
        "SPRITE \t(POS: {:?}; CYC: {:?})\t\t: {:?}",
        position, cycle_count, zero_vec
    );
}

fn draw_pixel(cycle_count: i32, sprite_center: i32) {
    let upp_bound: Vec<u32> = (1..240).step_by(40).collect();
    let down_bound: Vec<u32> = (40..=240).step_by(40).collect();
    for du in down_bound.iter().zip(upp_bound.iter()) {
        let (d, u) = du;
        if (cycle_count >= *d as i32) & (cycle_count <= *u as i32) {
        } else {
        }
    }

    println!("{:?}", upp_bound);
    println!("{:?}", down_bound);
}

fn part_2() {
    let lines = lines_from_file("./example.txt");

    let mut register_value = 1;
    let mut cycle_count = 0;
    let mut signal_strength = 0;
    let mut sprite_center = 1;

    for line in lines {
        if line == "noop" {
            // inspect_signal(cycle_count, register_value);
            draw_sprite(sprite_center, cycle_count);
            draw_pixel(cycle_count, sprite_center);
            cycle_count += 1;
            continue;
        }

        if line.contains("addx") {
            let (_, count) = line.split_once(" ").unwrap();
            let count = count.parse::<i32>().unwrap();

            cycle_count += 1;
            draw_sprite(sprite_center, cycle_count);
            cycle_count += 1;
            sprite_center += count;
            draw_sprite(sprite_center, cycle_count);
            continue;
        }
    }
    // inspect_signal(cycle_count, register_value);
    //
}
fn main() {
    // part_1();
    // inspect_signal(140, 3123);
    // part_2();
    draw_pixel(32, 32);
}
