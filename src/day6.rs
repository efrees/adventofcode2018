extern crate adventlib;

use regex::Regex;
use std::collections::*;

#[derive(Copy, Debug)]
struct FillPoint {
    x: u16,
    y: u16,
    d: u16,
    seed_id: u16,
}

impl Clone for FillPoint {
    fn clone(&self) -> FillPoint {
        *self
    }
}

impl FillPoint {
    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = vec![
            FillPoint {
                x: self.x + 1,
                y: self.y,
                d: self.d + 1,
                seed_id: self.seed_id,
            },
            FillPoint {
                x: self.x,
                y: self.y + 1,
                d: self.d + 1,
                seed_id: self.seed_id,
            },
        ];
        if self.x > 0 {
            neighbors.push(FillPoint {
                x: self.x - 1,
                y: self.y,
                d: self.d + 1,
                seed_id: self.seed_id,
            });
        }
        if self.y > 0 {
            neighbors.push(FillPoint {
                x: self.x,
                y: self.y - 1,
                d: self.d + 1,
                seed_id: self.seed_id,
            });
        }
        return neighbors;
    }

    fn is_in_boundaries(&self, max_x: u16, max_y: u16) -> bool {
        return self.x <= max_x && self.y <= max_y;
    }
}

pub fn solve() {
    println!("Day 6");

    let lines = adventlib::read_input_lines("day6input.txt");

    let mut fill_frontier: VecDeque<_> = lines
        .iter()
        .enumerate()
        .map(|(i, s)| parse_point(&s, i as u16))
        .collect();

    let max_x = fill_frontier.iter().map(|p| p.x).max().unwrap();
    let max_y = fill_frontier.iter().map(|p| p.y).max().unwrap();
    let point_count = fill_frontier.len();
    let tie_val = (point_count + 1) as u16;

    let mut grid = HashMap::new();
    let mut candidates: HashMap<_, _> = fill_frontier.iter().map(|p| (p.seed_id, 0)).collect();
    let mut infinite_candidates = HashSet::new();

    // Idea: fill from each point, breadth first
    while !fill_frontier.is_empty() {
        let point = fill_frontier.pop_front().unwrap();
        grid.entry((point.x, point.y))
            .and_modify(|cur: &mut FillPoint| {
                if cur.d > point.d {
                    // found a better one.
                    cur.d = point.d;
                    cur.seed_id = point.seed_id;
                    for next in point.neighbors() {
                        if next.is_in_boundaries(max_x, max_y) {
                            fill_frontier.push_back(next);
                        }
                    }
                } else if cur.d == point.d && cur.seed_id != point.seed_id {
                    cur.seed_id = tie_val;
                }
            }).or_insert_with(|| {
                for next in point.neighbors() {
                    if next.is_in_boundaries(max_x, max_y) {
                        fill_frontier.push_back(next);
                    }
                }
                return point;
            });
    }

    // for i in 0..400 {
    //     for j in 0..400 {
    //         match grid.get(&(i, j)) {
    //             Some(point) => print!("|{}", point.seed_id),
    //             None => (),
    //         }
    //     }
    //     println!();
    // }

    // Pass through the grid again, eliminating any candidates touching the perimeter.
    // Then look for the largest remaining fill
    for (_, point) in grid.iter() {
        // println!("Debug: {:#?}", point);
        candidates.entry(point.seed_id).and_modify(|v| *v += 1);
        if point.x == 0 || point.y == 0 || point.x == max_x || point.y == max_y {
            infinite_candidates.insert(point.seed_id);
        }
    }

    let largest_fill = candidates
        .iter()
        .filter(|(id, _v)| !infinite_candidates.contains(&id))
        .map(|(_id, val)| val)
        .max();

    println!("Max finite fill: {}", largest_fill.unwrap());
}

fn parse_point(line: &str, index: u16) -> FillPoint {
    lazy_static! {
        static ref point_pattern: Regex = Regex::new(r"^(\d+), (\d+)$").unwrap();
    }

    let captures = point_pattern.captures(line).unwrap();
    return FillPoint {
        x: captures[1].parse().unwrap(),
        y: captures[2].parse().unwrap(),
        d: 0,
        seed_id: index + 1, //save zero
    };
}
