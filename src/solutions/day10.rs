use regex::Regex;
use std::collections::*;

struct Point(i32, i32);

struct PointWithVelocity {
    point: Point,
    velocity: Point,
}

pub fn solve() {
    println!("Day 10");

    let lines = adventlib::read_input_lines("day10input.txt");
    let input_pattern =
        Regex::new(r"position=<\s*([-\d]+),\s*([-\d]+)> velocity=<\s*([-\d]+),\s*([-\d]+)>")
            .unwrap();

    let mut points: Vec<_> = lines
        .iter()
        .map(|line| input_pattern.captures(line).unwrap())
        .map(|caps| PointWithVelocity {
            point: Point(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            velocity: Point(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        })
        .collect();

    let mut prev_min_x = -999999;
    let mut prev_min_y = -999999;
    let mut prev_max_x = 999999;
    let mut prev_max_y = 999999;
    let mut still_converging = true;
    let mut step_count = 0;
    let step_size = 1;
    let target_y_diff = 15;
    while still_converging {
        let mut min_x = 999999;
        let mut min_y = 999999;
        let mut max_x = -999999;
        let mut max_y = -999999;
        for point in points.iter_mut() {
            let new_x = point.point.0 + step_size * point.velocity.0;
            let new_y = point.point.1 + step_size * point.velocity.1;
            if new_x < min_x {
                min_x = new_x;
            }
            if new_x > max_x {
                max_x = new_x;
            }
            if new_y < min_y {
                min_y = new_y;
            }
            if new_y > max_y {
                max_y = new_y;
            }
            point.point.0 = new_x;
            point.point.1 = new_y;
        }
        step_count += 1;

        if max_y - min_y < target_y_diff || (max_y - min_y) > (prev_max_y - prev_min_y) {
            still_converging = false;
        }

        prev_min_x = min_x;
        prev_min_y = min_y;
        prev_max_x = max_x;
        prev_max_y = max_y;
    }

    if prev_max_x - prev_min_x < 100 {
        print_point_section(prev_min_x, prev_min_y, prev_max_x, prev_max_y, &points);
    }

    println!("Convergence took {} steps", step_count);
}

fn print_point_section(
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    points: &Vec<PointWithVelocity>,
) {
    for j in min_y..=max_y {
        let xs_set: HashSet<_> = points
            .iter()
            .filter(|p| p.point.1 == j)
            .map(|p| p.point.0)
            .collect();
        let mut out_line = String::with_capacity((max_x - min_x) as usize);
        for i in min_x..=max_x {
            if xs_set.contains(&i) {
                out_line.push('#');
            } else {
                out_line.push(' ');
            }
        }
        println!("{}", out_line);
    }
}
