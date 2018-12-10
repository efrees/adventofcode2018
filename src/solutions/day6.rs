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
    #[inline]
    fn dist_to(&self, other_x: u16, other_y: u16) -> u16 {
        return ((self.x as i32 - other_x as i32).abs() + (self.y as i32 - other_y as i32).abs())
            as u16;
    }
}

pub fn solve() {
    println!("Day 6");

    let lines = adventlib::read_input_lines("day6input.txt");

    let seed_points: VecDeque<_> = lines
        .iter()
        .enumerate()
        .map(|(i, s)| parse_point(&s, i as u16))
        .collect();

    let max_x = seed_points.iter().map(|p| p.x).max().unwrap();
    let max_y = seed_points.iter().map(|p| p.y).max().unwrap();

    let mut grid = HashMap::new();

    // pre-compute all distances to locations within the bounding box
    for i in 0..=max_x {
        for j in 0..=max_y {
            for seed_point in seed_points.iter() {
                let dist = seed_point.dist_to(i, j);
                grid.entry((i, j))
                    .and_modify(|cur: &mut HashSet<_>| {
                        cur.insert((seed_point.seed_id, dist));
                    }).or_insert_with(|| {
                        let mut set = HashSet::new();
                        set.insert((seed_point.seed_id, dist));
                        return set;
                    });
            }
        }
    }

    // Part 1
    let mut all_closest = Vec::new();
    for (coords, set) in grid.iter() {
        let min_pair = set.iter().min_by_key(|(_id, d)| d).unwrap();
        let tied_count = set.iter().filter(|v| v.1 == min_pair.1).count();
        if tied_count == 1 {
            all_closest.push((coords.0, coords.1, min_pair.0));
        }
    }

    let infinite_seeds: HashSet<_> = all_closest
        .iter()
        .filter(|(x, y, _id)| *x == 0 || *y == 0 || *x == max_x || *y == max_y)
        .map(|(_, _, id)| *id)
        .collect();

    let mut region_sizes = HashMap::new();
    for (_x, _y, id) in all_closest.iter() {
        if infinite_seeds.contains(&id) {
            continue;
        }
        region_sizes.entry(id).and_modify(|x| *x += 1).or_insert(1);
    }

    let largest_fill = region_sizes.values().max().unwrap();

    println!("Max finite fill: {}", largest_fill);

    // Part 2
    let total_distance_thresh = 10_000;
    let points_in_common_region = grid
        .iter()
        .map(|(_coords, set): (&(u16, u16), &HashSet<(u16, u16)>)| {
            set.iter().map(|(_id, d)| *d as u32).sum()
        }).filter(|d: &u32| *d < total_distance_thresh)
        .count();

    println!("Points in common region: {}", points_in_common_region);
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
