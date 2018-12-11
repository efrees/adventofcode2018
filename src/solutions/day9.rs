use adventlib::collections::CircleList;
use std::collections::HashMap;

// My given values
#[allow(dead_code)]
fn day9_input() -> (u32, u32) {
    (458, 72019)
}

#[allow(dead_code)]
fn day9_test_input() -> (u32, u32) {
    (13, 7999)
}

pub fn solve() {
    println!("Day 9");

    let (num_players, mut last_marble_val) = day9_input();

    let mut circle = CircleList::with_capacity((last_marble_val / 2) as usize);
    circle.insert(0);

    let mut cur_player = 0;
    let mut current_marble = circle.last.unwrap();
    let mut next_marble_value = 1;
    let mut scores = HashMap::<u32, u32>::new();
    let mut turn_score;

    for part in 1..=2 {
        while next_marble_value <= last_marble_val {
            if next_marble_value % 23 == 0 {
                let mut search_marble = current_marble;
                for _ in 0..7 {
                    search_marble = circle.prev_node(search_marble);
                }
                current_marble = circle.next_node(search_marble);
                turn_score = next_marble_value + circle.remove(search_marble);
            } else {
                let next = circle.next_node(current_marble);
                circle.insert_after(next, next_marble_value);
                current_marble = circle.next_node(next);
                turn_score = 0;
            }

            scores
                .entry(cur_player)
                .and_modify(|s| *s += turn_score)
                .or_insert(turn_score);

            next_marble_value += 1;
            cur_player = (cur_player + 1) % num_players;
        }

        println!(
            "High score (part {}): {}",
            part,
            scores.values().max().unwrap()
        );

        last_marble_val *= 100;
    }
}
