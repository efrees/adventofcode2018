use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
// use std::str::*;

fn main() {
    solve_day_1();
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
