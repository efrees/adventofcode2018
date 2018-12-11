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
    let mut cur_len = cur_string.len();
    let mut next_len = 0;

    while replaced_one {
        replaced_one = false;
        let mut skip_one = false;
        let mut copy_to = 0;
        for i in 0..cur_len {
            if skip_one {
                skip_one = false;
                continue;
            }
            let cur_val = cur_string[i];
            if i + 1 < cur_len && (cur_val as i16 - cur_string[i + 1] as i16).abs() == 32 {
                skip_one = true;
                replaced_one = true;
                continue;
            }

            if copy_to < i {
                cur_string[copy_to] = cur_val;
            }
            copy_to += 1;
            next_len = copy_to;
        }

        cur_len = next_len;
    }

    cur_string.truncate(cur_len);
    return cur_string;
}
