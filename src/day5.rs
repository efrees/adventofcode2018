extern crate adventlib;

use std::collections::HashMap;

pub fn solve() {
    println!("Day 5");

    let lines = adventlib::read_input_lines("day5input.txt");
    let line = &lines[0];

    let lowercase_letters = "abcdefghijklmnopqrstuvwxyz";
    let mut uppercase_map = HashMap::<char, char>::new();

    // input is ASCII only
    for c in lowercase_letters.chars() {
        uppercase_map.insert(c, c.to_uppercase().next().unwrap());
    }

    let current_string = react_string(line, &uppercase_map);

    println!("Final length (part 1): {}", current_string.len());

    let mut min_length = current_string.len();

    for filter_c in lowercase_letters.chars() {
        let upper_filter_c = uppercase_map.get(&filter_c).unwrap();
        let filtered_line: String = current_string
            .chars()
            .filter(|c| c != &filter_c && c != upper_filter_c)
            .collect();

        let reacted_string = react_string(&filtered_line, &uppercase_map);

        if reacted_string.len() < min_length {
            min_length = reacted_string.len();
        }
    }

    println!("Best Length (part 2): {}", min_length);
}

fn react_string(line: &str, uppercase_map: &HashMap<char, char>) -> String {
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
                    if (uppercase_map.contains_key(&c) && uppercase_map.get(&c).unwrap() == &nextc)
                        || (uppercase_map.contains_key(&nextc)
                            && uppercase_map.get(&nextc).unwrap() == &c)
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

    return current_string;
}
