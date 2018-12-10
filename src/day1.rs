use std::collections::*;

pub fn solve() {
    println!("Day 1");

    let lines = adventlib::read_input_lines("day1input.txt");

    let mut total = 0;
    let mut seen = HashSet::new();
    seen.insert(total);
    let mut first_repeated = 0;
    let mut searching = true;

    let int_parser = |x: &String| x.parse::<i32>().unwrap();
    let diffs = lines.iter().map(int_parser);
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
        let diffs = lines.iter().map(int_parser);
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
