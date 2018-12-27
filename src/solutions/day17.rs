use regex::Regex;
use std::collections::*;

const SAND: u8 = 0;
const TOUCHED: u8 = 1;
const WATER: u8 = 2;
const CLAY: u8 = 3;

pub fn solve() {
    println!("Day 17");

    let lines = adventlib::read_input_lines("day17input.txt");
    let mut grid = HashMap::<(u32, u32), u8>::new();

    let mut min_y = 500;
    let mut max_y = 0;

    for line in lines.iter() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut x_part = parts[0];
        let mut y_part = parts[1];

        if x_part.as_bytes()[0] == b'y' {
            x_part = parts[1];
            y_part = parts[0];
        }

        let x_bounds = parse_bounds(x_part);
        let y_bounds = parse_bounds(y_part);

        if y_bounds.0 < min_y {
            min_y = y_bounds.0;
        }

        if y_bounds.1 > max_y {
            max_y = y_bounds.1;
        }

        for x in x_bounds.0..=x_bounds.1 {
            for y in y_bounds.0..=y_bounds.1 {
                grid.insert((x, y), CLAY);
            }
        }
    }

    let total_reached = flood_down(&mut grid, (500, 0), min_y, max_y);

    // print_grid(&grid, (300, min_y), (700, max_y));
    println!("Total squares touched: {}", total_reached);

    let total_retained = grid.values().filter(|&&x| x == WATER).count();
    println!("Total squares retained: {}", total_retained);
}

fn parse_bounds(specifier: &str) -> (u32, u32) {
    lazy_static! {
        static ref pattern: Regex =
            Regex::new(r"=(?P<start>\d+)(..(?P<end>\d+))?").expect("Specifier pattern");
    }

    let captures = pattern.captures(specifier).expect("Successful captures");
    let start = captures["start"].parse().unwrap();
    let end = if captures.name("end") != None {
        captures["end"].parse().unwrap()
    } else {
        start
    };

    (start, end)
}

fn flood_down(
    grid: &mut HashMap<(u32, u32), u8>,
    from_loc: (u32, u32),
    min_y: u32,
    max_y: u32,
) -> u32 {
    let mut total_touched = 0;

    if from_loc.1 > max_y {
        return 0;
    }

    if grid_get(grid, from_loc) > SAND {
        // already counted or clay
        return 0;
    }

    if from_loc.1 >= min_y {
        total_touched += 1;
    }

    let below = (from_loc.0, from_loc.1 + 1);

    total_touched += flood_down(grid, below, min_y, max_y);

    // spread if supported below
    if grid_get(grid, below) >= WATER {
        grid.insert(from_loc, WATER);
        total_touched += flood_left_right(grid, from_loc, max_y);
    } else {
        grid.insert(from_loc, TOUCHED);
    }

    return total_touched;
}

fn flood_left_right(grid: &mut HashMap<(u32, u32), u8>, from_loc: (u32, u32), max_y: u32) -> u32 {
    let mut total_touched = 0;
    let mut empties_out = false;

    let mut next = (from_loc.0 - 1, from_loc.1);
    let mut below = (from_loc.0, from_loc.1 + 1);
    while grid_get(grid, next) < WATER && grid_get(grid, below) >= WATER {
        grid.insert(next, WATER);
        if grid_get(grid, next) != TOUCHED {
            total_touched += 1;
        }
        below = (next.0, next.1 + 1);
        next = (next.0 - 1, next.1);

        total_touched += flood_down(grid, below, from_loc.1, max_y);

        if grid_get(grid, below) < WATER {
            empties_out = true;
        }
    }

    next = (from_loc.0 + 1, from_loc.1);
    below = (from_loc.0, from_loc.1 + 1);
    while grid_get(grid, next) < WATER && grid_get(grid, below) >= WATER {
        grid.insert(next, WATER);
        if grid_get(grid, next) != TOUCHED {
            total_touched += 1;
        }
        below = (next.0, next.1 + 1);
        next = (next.0 + 1, next.1);

        total_touched += flood_down(grid, below, from_loc.1, max_y);

        if grid_get(grid, below) < WATER {
            empties_out = true;
        }
    }

    if empties_out {
        empty_left_right(grid, from_loc);
    }

    return total_touched;
}

fn empty_left_right(grid: &mut HashMap<(u32, u32), u8>, from_loc: (u32, u32)) {
    grid.insert(from_loc, TOUCHED);

    let mut next = (from_loc.0 - 1, from_loc.1);
    while grid_get(grid, next) == WATER {
        grid.insert(next, TOUCHED);
        next = (next.0 - 1, next.1);
    }
    let mut next = (from_loc.0 + 1, from_loc.1);
    while grid_get(grid, next) == WATER {
        grid.insert(next, TOUCHED);
        next = (next.0 + 1, next.1);
    }
}

fn grid_get(grid: &HashMap<(u32, u32), u8>, loc: (u32, u32)) -> u8 {
    match grid.get(&loc) {
        Some(&x) => x,
        None => SAND,
    }
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<(u32, u32), u8>, top_left: (u32, u32), bottom_right: (u32, u32)) {
    for y in top_left.1..bottom_right.1 {
        for x in top_left.0..bottom_right.0 {
            let cell = grid_get(grid, (x, y));
            if cell == 0 {
                print!(" ");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}
