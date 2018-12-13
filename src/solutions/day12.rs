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

    let mut seen_patterns = HashMap::<String, (i64, i64)>::new();

    let mut next_config: Vec<_> = starting_config.iter().collect();
    let target_iterations = 50_000_000_000;
    let mut generation = 0;
    while generation < target_iterations {
        let blanks = vec![false; 3];
        zero_offset += 1;

        generation += 1;

        let starting_config: Vec<_> = blanks
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

        while next_config.iter().take(4).all(|&&x| !x) {
            next_config = next_config[1..].to_vec();
            zero_offset -= 1;
        }

        if generation == 20 {
            // Part 1
            print_result(&next_config, zero_offset, generation);
        }

        let state: String = next_config
            .iter()
            .map(|&&x| if x { return '#' } else { return '.' })
            .collect();

        if seen_patterns.contains_key(&state) {
            let prev_occurrence = seen_patterns.get(&state).expect("Key should be valid");
            let step_size = generation - prev_occurrence.0;
            let step_offset = zero_offset - prev_occurrence.1;

            let step_repetitions = (target_iterations - generation) / step_size;
            let total_offset = step_offset * step_repetitions;

            // jump forward
            generation += step_size * step_repetitions;
            zero_offset += total_offset;
        } else {
            seen_patterns.insert(state, (generation, zero_offset));
        }
    }

    //Part 2
    print_result(&next_config, zero_offset, generation);
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

fn print_result(next_config: &Vec<&bool>, zero_offset: i64, generation: i64) {
    let sum_of_locations: i64 = next_config
        .iter()
        .map(|&&x| x)
        .enumerate()
        .filter(|(_i, x)| *x)
        .map(|(i, _x)| i as i64 - zero_offset as i64)
        .sum();
    let count_of_locations = next_config.iter().filter(|&&&x| x).count();

    println!(
        "Sum of locations after step {}: {} ({})",
        generation, sum_of_locations, count_of_locations
    );
}
