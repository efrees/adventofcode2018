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

    // My given values
    let (num_players, last_marble_val) = day9_input();

    let mut cir = CircleList::new();
    cir.insert(0);
    let mut cur = cir.last.unwrap();
    cur = cir.next_node(cur);
    cir.insert_after(cur, 1);
    let one = cir.last.unwrap();
    assert_eq!(cur, cir.prev_node(one));
    cur = cir.next_node(one);
    cir.insert_after(cur, 2);
    print_circle(&cir);

    let mut circle = CircleList::new();
    circle.insert(0);

    let mut cur_player = 0;
    let mut current_marble = circle.last.unwrap();
    let mut next_marble_value = 1;
    let mut scores = HashMap::<u32, u32>::new();
    let mut turn_score;

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

    println!("High score: {}", scores.values().max().unwrap())
}

fn print_circle(circle: &CircleList<u32>) {
    let first_marble = circle.last.unwrap();
    print!("{} ", circle.get_value(first_marble).unwrap());
    let mut current_marble = circle.next_node(first_marble);
    while first_marble != current_marble {
        print!("{} ", circle.get_value(current_marble).unwrap());
        current_marble = circle.next_node(current_marble);
    }
    println!();
}
