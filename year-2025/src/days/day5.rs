use crate::days::util::read_file_lines;


pub fn solution() {
    let lines = read_file_lines("inputs/input5.txt");

    let index = lines.iter().position(|l| l.len() == 0).expect("found index");

    let ranges: Vec<Vec<u64>> = lines[..index]
    .iter()
    .map(|l| {
        let split: Vec<u64> = l.split("-").map(|s| s.parse().expect("an integer")).collect();
        return split;
    })
    .collect();

    let ids: Vec<u64> = lines[index+1..]
    .iter()
    .map(|s| s.parse()
    .expect("an id integer"))
    .collect();

    find_valid_ids(&ranges, ids); 

    total_valid_ids(&ranges);
}

fn find_valid_ids(ranges: &Vec<Vec<u64>>, ids: Vec<u64>) {
    let mut total = 0;

    for id in ids {
        for range in ranges {
            if id >= range[0] && id <= range[1] {
                total += 1;
                break;
            }
        } 
    }

    println!("{total}");
}

fn total_valid_ids(ranges: &Vec<Vec<u64>>) {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|a,b| a[0].cmp(&b[0]));

    let mut combined_ranges: Vec<Vec<u64>> = vec![sorted_ranges[0].clone()];

    for range in sorted_ranges[1..].iter() {
        let last_range = combined_ranges.last_mut().unwrap();

        //print!("\n{:?} -- {:?} - ", last_range, range);

        if range[0] >= last_range[0] && range[1] <= last_range[1] {
            //print!("skip ");
            continue;
        }

        if range[0] <= last_range[1] && range[1] > last_range[1] {
            //print!("max ");
            last_range[1] = range[1];
        }

        if range[0] < last_range[0] && range[1] >= last_range[0] {
            //print!("min ");
            last_range[0] = range[0];
        }

        if range[0] > last_range[1] {
            //print!("new");
            combined_ranges.push(range.clone());
        }
    }

    //println!("");

    let mut total = 0;
    for range in combined_ranges {
        total += range[1] - range[0] + 1;
    }
    println!("{total}");
}