extern crate adventlib;

use std::cmp::Ordering;
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
    let mut unresolved_deps_part2 = HashMap::<char, HashSet<char>>::new();
    let mut graph = HashMap::<char, HashSet<char>>::new();
    let mut result = Vec::<char>::new();
    for edge in edges.iter() {
        all_nodes.insert(edge.0);
        all_nodes.insert(edge.1);
        unresolved_deps.insert(edge.0, HashSet::new());
        unresolved_deps.insert(edge.1, HashSet::new());

        unresolved_deps_part2.insert(edge.0, HashSet::new());
        unresolved_deps_part2.insert(edge.1, HashSet::new());
    }

    for edge in edges.iter() {
        unresolved_deps.entry(edge.1).and_modify(|set| {
            set.insert(edge.0);
        });

        unresolved_deps_part2.entry(edge.1).and_modify(|set| {
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

    // Part 1
    while all_nodes.len() > result.len() {
        let next_available = unresolved_deps
            .iter()
            .filter(|(_k, v)| v.is_empty())
            .map(|(k, _v)| *k)
            .min()
            .unwrap();
        result.push(next_available);
        unresolved_deps.remove(&next_available);

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

    // Part 2
    let num_workers = 5;
    let mut elapsed_seconds = 0;
    let mut pending_nodes = BinaryHeap::<ReverseOrderedTuple>::new(); // finish time and name
    let mut result_part2 = Vec::<char>::new();

    while all_nodes.len() > result_part2.len() {
        // process finished ones
        while pending_nodes.len() > 0 && pending_nodes.peek().unwrap().0 <= elapsed_seconds {
            let node = pending_nodes.pop().unwrap();
            result_part2.push(node.1);

            let adjacent = match graph.get(&node.1) {
                Some(set) => set,
                None => continue,
            };
            for adj in adjacent {
                unresolved_deps_part2.entry(*adj).and_modify(|s| {
                    s.remove(&node.1);
                });
            }
        }

        // if someone's available, try to find them work
        while pending_nodes.len() < num_workers {
            let next_available = match unresolved_deps_part2
                .iter()
                .filter(|(_k, v)| v.is_empty())
                .map(|(k, _v)| *k)
                .min()
            {
                Some(c) => c,
                None => break,
            };

            unresolved_deps_part2.remove(&next_available);
            pending_nodes.push(ReverseOrderedTuple(
                elapsed_seconds + node_cost(&next_available),
                next_available,
            ));
        }

        // advance time
        elapsed_seconds += 1;
    }

    elapsed_seconds -= 1; // We prepped for a loop we didn't need

    println!("Elapsed seconds: {}", elapsed_seconds);
}

fn node_cost(node: &char) -> u16 {
    let mut b = [0; 2];
    let alpha_offset = 0x40;

    node.encode_utf8(&mut b);
    return b[0] as u16 - alpha_offset + 60;
}

#[derive(PartialEq, Eq)]
struct ReverseOrderedTuple(u16, char);

impl PartialOrd for ReverseOrderedTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // reverse for use in
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for ReverseOrderedTuple {
    fn cmp(&self, other: &ReverseOrderedTuple) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
