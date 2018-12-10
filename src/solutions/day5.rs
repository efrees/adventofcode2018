extern crate adventlib;

pub fn solve() {
    println!("Day 5");

    let lines = adventlib::read_input_lines("day5input.txt");
    let line: Vec<_> = lines[0].as_bytes().iter().map(|x| *x).collect();

    let current_string = react_string(line);

    println!("Final length (part 1): {}", current_string.len());

    let mut min_length = current_string.len();

    for letter_idx in 0..26 {
        let filtered_line: Vec<_> = current_string
            .iter()
            .filter(|c| **c != b'a' + letter_idx && **c != b'A' + letter_idx)
            .map(|x| *x)
            .collect();

        let reacted_string = react_string(filtered_line);

        if reacted_string.len() < min_length {
            min_length = reacted_string.len();
        }
    }

    println!("Best Length (part 2): {}", min_length);
}

fn react_string(line: Vec<u8>) -> Vec<u8> {
    let mut replaced_one = true;
    let mut cur_string = line;
    let mut new_string = Vec::with_capacity(cur_string.len() / 2);

    while replaced_one {
        replaced_one = false;
        let mut skip_one = false;
        for i in 0..cur_string.len() {
            if skip_one {
                skip_one = false;
                continue;
            }

            if i == cur_string.len() - 1 {
                new_string.push(cur_string[i]);
            } else if (cur_string[i] as i16 - cur_string[i + 1] as i16).abs() == 32 {
                skip_one = true;
                replaced_one = true;
            } else {
                new_string.push(cur_string[i]);
            }
        }

        cur_string = new_string;
        new_string = Vec::with_capacity(cur_string.len() / 2);
    }

    return cur_string;
}
