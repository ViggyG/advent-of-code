use crate::days::util::{read_file_lines};

pub fn solution() {
    let file_lines = read_file_lines("inputs/input1.txt");

    let mut position = 50;
    let mut total = 0;
    let mut total_passes = 0;

    for line in &file_lines {
        let sign = if line.as_bytes()[0] == b'L' {
            -1
        } else {
            1
        };

        let value: i32 =  line[1..].parse().expect("to find an integer");

        let rotation = (value % 100) * sign;

        let passes = value/100;

        total_passes += passes;

        let prev_pos = position;

        position += rotation;

        if position > 99 || position < 0 {
            position += 100 * sign * -1;
            if prev_pos != 0 && position != 0 {
                total_passes += 1;
            }
        }

        if position == 0 {
            total_passes += 1;
            total += 1;
        }
    }

    println!("day 1 - total: {total} | total passes: {total_passes}");
}