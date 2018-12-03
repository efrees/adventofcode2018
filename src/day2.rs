use std::collections::*;
use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
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
