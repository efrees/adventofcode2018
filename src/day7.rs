extern crate adventlib;

use regex::Regex;
use std::collections::*;

pub fn solve() {
    println!("Day 7");

    let lines = adventlib::read_input_lines("day7input.txt");

    // example line: 'Step G must be finished before step V can begin.'
    let edges: Vec<_> = lines
        .iter()
        .map(|l| (l.chars().nth(5).unwrap(), l.chars().nth(36).unwrap()))
        .collect();
    let mut all_nodes = HashSet::new();
    let mut unresolved_deps = HashMap::<char, HashSet<char>>::new();
    let mut graph = HashMap::<char, HashSet<char>>::new();
    let mut result = Vec::<char>::new();
    for edge in edges.iter() {
        all_nodes.insert(edge.0);
        all_nodes.insert(edge.1);
        unresolved_deps.insert(edge.0, HashSet::new());
        unresolved_deps.insert(edge.1, HashSet::new());
    }

    for edge in edges.iter() {
        unresolved_deps.entry(edge.1).and_modify(|set| {
            set.insert(edge.0);
        });

        graph
            .entry(edge.0)
            .and_modify(|set| {
                set.insert(edge.1);
            }).or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(edge.1);
                return set;
            });
    }

    while all_nodes.len() > result.len() {
        let next_available = unresolved_deps
            .iter()
            .filter(|(_k, v)| v.is_empty())
            .map(|(k, _v)| *k)
            .min()
            .unwrap();
        result.push(next_available);
        unresolved_deps.remove(&next_available);
        // println!("Debug next {}", next_available);
        let adjacent = match graph.get(&next_available) {
            Some(set) => set,
            None => continue,
        };
        for adj in adjacent {
            unresolved_deps.entry(*adj).and_modify(|s| {
                s.remove(&next_available);
            });
        }
    }

    let result: String = result.iter().collect();
    println!("Sorted elements: {}", result);
}
