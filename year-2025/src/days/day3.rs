use std::{collections::VecDeque, ops::Index};

use crate::days::util::read_file_lines;


pub fn solution() {
    let lines = read_file_lines("inputs/input3.txt");

    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    for line in &lines {
        //total_part_1 += test_str(line, 2);
        //total_part_2 += test_str(line, 12);
    }

    //test_str(&String::from("987654321111111"), 12);
    //test_str(&String::from("811111111111119"), 12);
    //test_str(&String::from("234234234234278"), 12);
    test_str(&String::from("818181911112111"), 12);
    println!("{total_part_1}");
    println!("{total_part_2}");
}

fn test_str(str: &String, batch_size: u32) -> u64 {
    let values: Vec<u32> = str
    .chars()
    .map(|v| v.to_string().parse().expect("an integer"))
    .collect();

    let mut sorted = values.clone();
    sorted.sort();
    sorted.reverse();

    let mut skips = 0;
    let skips_required: u32 = str.len() as u32 - batch_size;

    let mut i: usize = 0;

    let output: Vec<u32> = vec![];

    for v in values[0..].iter() {
        let next_index = (i+1).clamp(0, str.len()-1);
        let next_val = values[next_index];

        

        i += 1;
    }

    return 0;
}

struct StrongSeries {
    weak_indeces: VecDeque<usize>,
    size: usize,
    data: VecDeque<u32>
}

impl StrongSeries {
    fn new(size: usize) -> Self {
        StrongSeries {
            weak_indeces: vec![].into(),
            size: size,
            data: vec![].into()
        }
    }

    fn add(mut self, value: u32) {
        let len = self.data.len();

        if len == 0 {
            self.data.push_back(value);
            return;
        }

        let last_val = self.data[len-1];

        if last_val < value {

        }
    }
}