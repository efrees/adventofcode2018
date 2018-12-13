use std::collections::HashMap;

pub fn solve() {
    println!("Day 12");

    let lines = adventlib::read_input_lines("day12input.txt");

    let mut zero_offset = 0;
    let input_label = "initial state: ";
    let starting_config: Vec<bool> = lines[0][input_label.len()..]
        .chars()
        .map(|c| c == '#')
        .collect();
    let rules: HashMap<_, bool> = lines[2..].iter().map(parse_rule).collect();

    let mut next_config: Vec<_> = starting_config.iter().collect();

    for _ in 0..20 {
        let blanks = vec![false; 3];
        zero_offset += 1;

        let mut starting_config: Vec<_> = blanks
            .iter()
            .chain(next_config.iter().map(|x| *x))
            .chain(blanks.iter())
            .collect();

        next_config = starting_config
            .iter()
            .zip(starting_config.iter().skip(1))
            .zip(starting_config.iter().skip(2))
            .zip(starting_config.iter().skip(3))
            .zip(starting_config.iter().skip(4))
            .map(|((((&a, &b), &c), &d), &e)| (a, b, c, d, e))
            .map(|(&a, &b, &c, &d, &e)| match rules.get(&(a, b, c, d, e)) {
                Some(outcome) => outcome,
                None => &false,
            }).collect();
    }

    let sum_of_locations: i32 = next_config
        .iter()
        .map(|&&x| x)
        .enumerate()
        .filter(|(_i, x)| *x)
        .map(|(i, _x)| i as i32 - zero_offset as i32)
        .sum();
    let count_of_locations = next_config.iter().filter(|&&&x| x).count();

    println!(
        "Sum of locations after {}: {} ({})",
        20, sum_of_locations, count_of_locations
    );
}

fn parse_rule(line: &String) -> ((bool, bool, bool, bool, bool), bool) {
    // example '...## => #'
    //          0123456789
    let line = line.as_bytes();
    let left_side = (
        line[0] == b'#',
        line[1] == b'#',
        line[2] == b'#',
        line[3] == b'#',
        line[4] == b'#',
    );
    let right_side = line[9] == b'#';

    return (left_side, right_side);
}
