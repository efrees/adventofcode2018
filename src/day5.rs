extern crate adventlib;

use std::collections::*;

pub fn solve() {
    println!("Day 5");

    let lines = adventlib::read_input_lines("day5input.txt");
    let line = &lines[0];

    // input is ASCII only
    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let letter_pairs: Vec<_> = lowercase_letters
        .chars()
        .zip(uppercase_letters.chars())
        .collect();
    let pairs_used = HashMap::<(char, char), u32>::new();
    let mut replaced_one = true;
    let mut current_string = line.to_string();
    let mut new_string = String::new();

    while replaced_one {
        replaced_one = false;
        let mut skip_one = false;
        for (i, c) in current_string.char_indices() {
            if skip_one {
                skip_one = false;
                continue;
            }

            let next_char = current_string[i + 1..].chars().next();
            match next_char {
                Some(nextc) => {
                    if letter_pairs.contains(&(c, nextc)) || letter_pairs.contains(&(nextc, c))
                    // if lowercase_letters.find(c) == uppercase_letters.find(nextc)
                    //     || lowercase_letters.find(nextc) == uppercase_letters.find(c)
                    {
                        skip_one = true;
                        replaced_one = true;
                    } else {
                        new_string.push(c);
                    }
                }
                None => new_string.push(c),
            }
        }
        current_string = new_string;
        new_string = String::new();
    }

    println!("Final length: {}", current_string.len());
}
