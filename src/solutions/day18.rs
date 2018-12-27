use adventlib::grid::*;
use std::collections::HashMap;

const OPEN: u8 = 0;
const TREES: u8 = 1;
const LUMBERYARD: u8 = 2;

pub fn solve() {
    println!("Day 18");

    let lines = adventlib::read_input_lines("day18input.txt");

    let mut grid = lines_to_byte_grid(&lines);
    let mut next_grid = lines_to_byte_grid(&lines);

    for _ in 0..10 {
        compute_next_step(&grid, &mut next_grid);
        let temp = grid;
        grid = next_grid;
        next_grid = temp;
    }

    println!("Resource value after 10: {}", compute_resource_value(&grid));

    let mut seen = HashMap::new();
    let target_minutes = 1_000_000_000;
    let mut count = 10;
    while count < target_minutes {
        compute_next_step(&grid, &mut next_grid);
        let string_rep = grid_to_string(&next_grid);
        if seen.contains_key(&string_rep) {
            let first_minute = seen.get(&string_rep).expect("Get of confirmed key");
            let step_size = count - first_minute;
            let remainder = (target_minutes - count) % step_size;
            count = target_minutes - remainder;
        }
        seen.insert(string_rep, count);

        let temp = grid;
        grid = next_grid;
        next_grid = temp;

        count += 1;
    }

    println!(
        "Resource value after {}: {}",
        target_minutes,
        compute_resource_value(&grid)
    );
}

fn lines_to_byte_grid(lines: &Vec<String>) -> Vec<Vec<u8>> {
    let mut grid_rows = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        let mut row: Vec<_> = line
            .as_bytes()
            .iter()
            .map(|x| match x {
                b'|' => TREES,
                b'#' => LUMBERYARD,
                _ => OPEN,
            })
            .collect();
        grid_rows.push(row);
    }
    return grid_rows;
}

fn compute_next_step(grid: &Vec<Vec<u8>>, next_grid: &mut Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let point = Point::new(j as i64, i as i64);
            let tree_and_yard_counts = count_neighbors(point, &grid);

            if grid[i][j] == OPEN {
                next_grid[i][j] = if tree_and_yard_counts.0 >= 3 {
                    TREES
                } else {
                    OPEN
                };
            } else if grid[i][j] == TREES {
                next_grid[i][j] = if tree_and_yard_counts.1 >= 3 {
                    LUMBERYARD
                } else {
                    TREES
                };
            } else if grid[i][j] == LUMBERYARD {
                next_grid[i][j] = if tree_and_yard_counts.0 >= 1 && tree_and_yard_counts.1 >= 1 {
                    LUMBERYARD
                } else {
                    OPEN
                };
            }
        }
    }
}

fn count_neighbors(point: Point, grid: &Vec<Vec<u8>>) -> (u8, u8) {
    let neighbors = point.neighbors8();
    let tree_count = neighbors
        .iter()
        .filter(|&n| grid_get(n, grid) == TREES)
        .count() as u8;
    let yard_count = neighbors
        .iter()
        .filter(|&n| grid_get(n, grid) == LUMBERYARD)
        .count() as u8;
    return (tree_count, yard_count);
}

fn grid_get(point: &Point, grid: &Vec<Vec<u8>>) -> u8 {
    if point.y < 0 || point.y as usize >= grid.len() {
        return OPEN;
    }
    if point.x < 0 || point.x as usize >= grid[point.y as usize].len() {
        return OPEN;
    }
    grid[point.y as usize][point.x as usize]
}

fn compute_resource_value(grid: &Vec<Vec<u8>>) -> u32 {
    let mut wood_count = 0;
    let mut yard_count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == TREES {
                wood_count += 1;
            } else if grid[i][j] == LUMBERYARD {
                yard_count += 1;
            }
        }
    }

    return wood_count * yard_count;
}

fn grid_to_string(grid: &Vec<Vec<u8>>) -> String {
    let mut string = String::with_capacity(2500);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            string.push(match grid[i][j] {
                1 => '|',
                2 => '#',
                _ => '.',
            });
        }
    }
    return string;
}
