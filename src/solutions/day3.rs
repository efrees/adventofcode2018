use regex::Regex;
use std::collections::*;

pub fn solve() {
    println!("Day 3");

    let mut grid = vec![[0; 1000]; 1000];
    let mut last_drawn = vec![[0; 1000]; 1000];
    let mut non_overlapped_ids = HashSet::<i32>::new();

    let lines = adventlib::read_input_lines("day3input.txt");

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in lines.iter() {
        let cap = re.captures_iter(line).next().unwrap();
        let id: i32 = cap[1].parse().unwrap();
        let x: i32 = cap[2].parse().unwrap();
        let y: i32 = cap[3].parse().unwrap();
        let w: i32 = cap[4].parse().unwrap();
        let h: i32 = cap[5].parse().unwrap();

        non_overlapped_ids.insert(id);
        for i in x..(x + w) {
            for j in y..(y + h) {
                if grid[i as usize][j as usize] != 0 {
                    non_overlapped_ids.remove(&last_drawn[i as usize][j as usize]);
                    non_overlapped_ids.remove(&id);
                }

                grid[i as usize][j as usize] += 1;
                last_drawn[i as usize][j as usize] = id;
            }
        }
    }

    let mut total_overlap = 0;
    for row in grid.iter() {
        for col_val in row.iter() {
            if col_val > &1 {
                total_overlap += 1;
            }
        }
    }

    assert!(non_overlapped_ids.len() == 1, "Should only have one left.");

    println!("Total overlap: {}", total_overlap);
    println!(
        "Non-overlapping id: {}",
        non_overlapped_ids.iter().next().unwrap()
    );
}
