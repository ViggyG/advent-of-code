use crate::days::util::read_file_lines;

pub fn solution() {
    let lines = read_file_lines("inputs/input2.txt");
    let id_ranges = lines[0].split(',');

    let mut id_total = 0;

    for id_range in id_ranges {
        let bounds: Vec<&str> = id_range.split('-').collect();
        let lower = bounds[0];
        let upper = bounds[1];
        
        let _lower_val: i64 = lower.parse().expect("an integer");
        let _upper_val: i64 = upper.parse().expect("an integer");

        //id_total += check_ids(lower_val, upper_val);
    }
    id_total = check_ids(11,22);
    println!("{id_total}");
}

fn check_ids(lower: i64, upper: i64) -> i64 {
    let mut total = 0;

    for id in lower..=upper {
        let id_str = id.to_string();
        let id_chars: Vec<char> = id_str.chars().collect();

        let mut is_symetric = true;

        for i in 0..id_str.len()/2 {
            //let section = String.from(id_str[0..i]);
        }

        if is_symetric {
            println!("{id_str}");
            total += id;
        }
    }

    total
}