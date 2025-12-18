use crate::days::util::read_file_lines;


pub fn solution() {
    let lines = read_file_lines("inputs/input4.txt");

    let mut grid: Vec<Vec<char>> = lines
    .into_iter()
    .map(|l| l.chars().collect())
    .collect();

    let total = total_accessible_blocks(&mut grid, 1, true);

    println!("{total}");

    let mut removed_this_iteration = 1;

    let mut total_possible_to_remove = total;

    while (removed_this_iteration > 0) {
        removed_this_iteration = total_accessible_blocks(&mut grid, 1, false);
        total_possible_to_remove += removed_this_iteration;
    }

    println!("{total_possible_to_remove}");
}

fn total_accessible_blocks(grid: &mut Vec<Vec<char>>, distance: i32, include_removed: bool) -> u32 {
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let mut found_blocks = 0;
            if grid[y][x] != '@' {
                continue;
            }
            for wy in -distance..distance + 1 {
                for wx in -distance..distance + 1 {
                    let cy = y as i32 + wy;
                    let cx = x as i32 + wx;
                    if cy < 0 || cy >= grid.len() as i32 {
                        continue;
                    }
                    if cx < 0 || cx >= grid[y].len() as i32 {
                        continue;
                    }

                    let char = grid[cy as usize][cx as usize];

                    if char == '@' || (include_removed && char == 'X') {
                        found_blocks += 1;
                    }
                }
            }
            if found_blocks < 5 {
                total += 1;
                grid[y][x] = 'X';
            }
        }
    }
    
    total
}