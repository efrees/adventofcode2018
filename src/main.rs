extern crate regex;

use regex::Regex;
use std::boxed::Box;
use std::collections::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // solve_day_1();
    // solve_day_2();
    solve_day_3();
}

fn solve_day_1() {
    println!("Day 1");

    let filename = "day1input.txt";

    // TODO: factor out function
    let filename = "inputs/".to_owned() + filename;
    let mut f = File::open(filename).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Problem reading the contents of the file.");

    let mut total = 0;
    let mut seen = HashSet::new();
    seen.insert(total);
    let mut first_repeated = 0;
    let mut searching = true;

    let lines = contents.split_whitespace();
    let int_parser = |x: &str| x.parse::<i32>().unwrap();
    let diffs = lines.map(int_parser);
    for diff in diffs {
        total += diff;

        if searching && seen.contains(&total) {
            searching = false;
            first_repeated = total;
        } else {
            seen.insert(total);
        }
    }

    println!("Total: {}", total);

    while searching {
        let lines = contents.split_whitespace();
        let diffs = lines.map(int_parser);
        for diff in diffs.into_iter() {
            total += diff;

            if searching && seen.contains(&total) {
                searching = false;
                first_repeated = total;
                break;
            }

            seen.insert(total);
        }
    }

    println!("First repeated: {}", first_repeated);
}

fn solve_day_2() {
    println!("Day 2");

    let filename = "day2input.txt";

    let filename = "inputs/".to_owned() + filename;
    let mut f = File::open(filename).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Problem reading the contents of the file.");

    let mut doubles_count = 0;
    let mut triples_count = 0;

    let lines = contents.split_whitespace();
    for line in lines {
        let mut counts = HashMap::<char, i32>::new();
        for c in line.chars() {
            counts.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        if counts.iter().any(|(_, y)| *y == 2) {
            doubles_count += 1;
        }

        if counts.iter().any(|(_, y)| *y == 3) {
            triples_count += 1;
        }
    }

    println!("Checksum: {}", doubles_count * triples_count);

    let lines: Vec<_> = contents.split_whitespace().collect();
    let mut matched_string = "".to_string();
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }

            let first_line = lines[i];
            let second_line = lines[j];

            if first_line.len() != second_line.len() {
                println!("Did not expect different line lengths.");
                continue;
            }

            let mut single_diff_found = false;
            let mut diff_found_at = 0;
            for (k, c) in first_line.chars().enumerate() {
                let second_line_char = second_line[k..].chars().next().unwrap();
                if second_line_char == c {
                    continue;
                }
                if !single_diff_found {
                    single_diff_found = true;
                    diff_found_at = k;
                } else {
                    single_diff_found = false;
                    break;
                }
            }

            if single_diff_found {
                matched_string = first_line[0..diff_found_at].to_string();
                matched_string.push_str(&first_line[(diff_found_at + 1)..]);
                break;
            }
        }

        if matched_string.len() > 0 {
            break;
        }
    }

    println!("Match without difference: {}", matched_string);
}

fn solve_day_3() {
    println!("Day 3");

    let mut grid = vec![[0; 1000]; 1000];
    let mut last_drawn = vec![[0; 1000]; 1000];
    let mut non_overlapped_ids = HashSet::<i32>::new();

    let filename = "day3input.txt";

    let filename = "inputs/".to_owned() + filename;
    let mut f = File::open(filename).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Problem reading the contents of the file.");

    let lines = contents.lines();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in lines {
        let cap = re.captures_iter(line).next().unwrap();
        let id: i32 = cap[1].parse().unwrap();
        let x: i32 = cap[2].parse().unwrap();
        let y: i32 = cap[3].parse().unwrap();
        let w: i32 = cap[4].parse().unwrap();
        let h: i32 = cap[5].parse().unwrap();

        non_overlapped_ids.insert(id);
        println!("Debug {} {} {} {}", x, y, w, h);
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
