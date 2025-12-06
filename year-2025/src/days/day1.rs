use crate::days::util::{read_file_lines};

pub fn solution() -> usize {
    let file_lines = read_file_lines("inputs/input1.txt");
    file_lines.len()
}