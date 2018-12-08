extern crate adventlib;

use std::collections::*;
use std::iter::*;

pub fn solve() {
    println!("Day 8");

    let data: Vec<u32> = adventlib::read_input_raw("day8input.txt")
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // Part 1
    let mut tree_copy: VecDeque<u32> = data.iter().map(|x| *x).collect();
    let total_metadata = sum_metadata_rec(&mut tree_copy);
    println!("Total of metadata: {}", total_metadata);

    // Part 2
    let mut tree_copy: VecDeque<u32> = data.iter().map(|x| *x).collect();
    let root_node_value = get_node_value(&mut tree_copy);
    println!("Root node value: {}", root_node_value);
}

fn sum_metadata_rec(tree_data: &mut VecDeque<u32>) -> u32 {
    let children: u32 = tree_data.pop_front().unwrap();
    let metadata_count: u32 = tree_data.pop_front().unwrap();
    let mut total_metadata = 0;
    for _ in 1..=children {
        total_metadata += sum_metadata_rec(tree_data);
    }

    for _ in 1..=metadata_count {
        total_metadata += tree_data.pop_front().unwrap();
    }
    return total_metadata;
}

fn get_node_value(tree_data: &mut VecDeque<u32>) -> u32 {
    let children: u32 = tree_data.pop_front().unwrap();
    let metadata_count: u32 = tree_data.pop_front().unwrap();
    let mut node_value = 0;

    if children == 0 {
        for _ in 1..=metadata_count {
            node_value += tree_data.pop_front().unwrap();
        }
    } else {
        let mut child_values = Vec::<u32>::new();
        for _ in 1..=children {
            child_values.push(get_node_value(tree_data));
        }
        for _ in 1..=metadata_count {
            let metadata_val = tree_data.pop_front().unwrap();
            assert!(metadata_val > 0);

            let child_index = (metadata_val - 1) as usize;

            if child_index < child_values.len() {
                node_value += child_values.get(child_index).unwrap();
            }
        }
    }
    return node_value;
}
