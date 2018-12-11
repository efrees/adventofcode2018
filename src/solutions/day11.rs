pub fn solve() {
    println!("Day 11");

    let puzzle_input = 7347;
    let mut grid = vec![vec![0; 300]; 300];

    // fill in values
    for i in 0..300 {
        for j in 0..300 {
            let x = i + 1;
            let y = j + 1;
            grid[i][j] = power_level(x as i32, y as i32, puzzle_input);
        }
    }

    let mut max_power_level = -100;
    let mut max_power_location = (0, 0);
    for i in 0..298 {
        for j in 0..298 {
            let x = i + 1;
            let y = j + 1;
            let total_power_level = grid[i][j]
                + grid[i + 1][j]
                + grid[i + 2][j]
                + grid[i][j + 1]
                + grid[i + 1][j + 1]
                + grid[i + 2][j + 1]
                + grid[i][j + 2]
                + grid[i + 1][j + 2]
                + grid[i + 2][j + 2];

            if total_power_level > max_power_level {
                max_power_level = total_power_level;
                max_power_location = (x, y);
            }
        }
    }

    println!(
        "Best 3x3 at {},{}",
        max_power_location.0, max_power_location.1
    );

    let mut max_power_level = -100;
    let mut max_power_location = (0, 0);
    let mut max_power_size = 1;
    for size in 1..300 {
        for i in 0..=(300 - size) {
            for j in 0..=(300 - size) {
                let mut total_power_level = 0;
                for k in 0..size {
                    for l in 0..size {
                        total_power_level += grid[i + k][j + l];
                    }
                }

                if total_power_level > max_power_level {
                    max_power_level = total_power_level;
                    max_power_location = (i + 1, j + 1);
                    max_power_size = size;
                }
            }
        }
    }

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
