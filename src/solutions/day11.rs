pub fn solve() {
    println!("Day 11");

    let puzzle_input = 7347;
    let mut grid = vec![vec![0; 300]; 300];

    let mut max_power_level = -100;
    let mut max_power_location = (0, 0);
    let mut max_power_size = 1;

    // base case
    for i in 0..300 {
        for j in 0..300 {
            let x = i + 1;
            let y = j + 1;
            grid[i][j] = power_level(x as i32, y as i32, puzzle_input);

            if grid[i][j] > max_power_level {
                max_power_level = grid[i][j];
                max_power_location = (i + 1, j + 1);
                max_power_size = 1;
            }
        }
    }

    let mut sized_grids = Vec::new();
    sized_grids.push(grid);

    let mut max_3x3_power = -100;
    let mut max_3x3_location = (0, 0);
    for size in 2..=300 {
        let result_size = 300 + 1 - size;
        let mut grid = Vec::<Vec<i32>>::with_capacity(result_size);

        let half_size = size / 2;
        for i in 0..=(300 - size) {
            grid.push(Vec::with_capacity(result_size));

            for j in 0..=(300 - size) {
                let half_size_grid = &sized_grids[half_size - 1];

                let mut total_power_level = 0;
                total_power_level += half_size_grid[i][j];
                total_power_level += half_size_grid[i + half_size][j];
                total_power_level += half_size_grid[i][j + half_size];
                total_power_level += half_size_grid[i + half_size][j + half_size];

                if size % 2 > 0 {
                    // odd row around right and bottom edges
                    for k in 0..size {
                        total_power_level += sized_grids[0][i + size - 1][j + k];
                        total_power_level += sized_grids[0][i + k][j + size - 1];
                    }
                    // remove overlap
                    total_power_level -= sized_grids[0][i + size - 1][j + size - 1];
                }

                if total_power_level > max_power_level {
                    max_power_level = total_power_level;
                    max_power_location = (i + 1, j + 1);
                    max_power_size = size;
                }

                if size == 3 && total_power_level > max_3x3_power {
                    max_3x3_power = total_power_level;
                    max_3x3_location = (i + 1, j + 1);
                }

                grid[i].push(total_power_level);
            }
        }

        //print_grid(&grid);
        sized_grids.push(grid);
    }

    println!("Best 3x3 at {},{}", max_3x3_location.0, max_3x3_location.1);

    println!(
        "Best of any size at {},{},{}",
        max_power_location.0, max_power_location.1, max_power_size
    );
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y + serial_number;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;
    return power_level;
}

#[allow(unused)]
fn print_grid(grid: &Vec<Vec<i32>>) {
    println!("----------");
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            print!(" {:02}", grid[i][j]);
        }
        println!();
    }
}
