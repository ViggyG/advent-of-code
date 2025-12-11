use crate::days::util::read_file_lines;


pub fn solution() {
    let lines = read_file_lines("inputs/input3.txt");

    let total = 0;

    // for line in &lines {
    //     let result = test_str(line);
    //     println!("{result}")
    // }

    let test = test_str(&String::from("987654321111111"));
    let test2 = test_str(&String::from("81119"));
    println!("{test}");
    println!("{test2}")
}

fn test_str(str: &String) -> u32 {
    let chars: Vec<char> = str.chars().collect();
    let mut pos_1: u32 = 0;
    let mut pos_2: u32 = 0;
    for c in chars {
        let value = c.to_digit(10).expect("a digit");
        if value > pos_2 {
            pos_1 = pos_2;
            pos_2 = value;
            continue;
        }

        if value > pos_1 {
            pos_1 = value;
        }
    }
    let value_string: u32 = format!("{pos_1}{pos_2}").parse().expect("an integer");
    return value_string;
}