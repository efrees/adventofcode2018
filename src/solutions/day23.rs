use adventlib;
use adventlib::grid::*;
use regex::Regex;

pub fn solve() {
    println!("Day 23");

    let lines = adventlib::read_input_lines("day23input.txt");

    let bots: Vec<_> = lines.iter().map(|l| parse_bot(l)).collect();
    let strongest_bot = bots.iter().max_by_key(|b| b.radius).expect("Must be a max");

    let bots_in_reach = bots
        .iter()
        .filter(|b| {
            b.location.manhattan_dist_to(&strongest_bot.location) <= strongest_bot.radius as i64
        })
        .count();

    println!("Bots reached by strongest: {}", bots_in_reach);
}

fn parse_bot(line: &str) -> NanoBot {
    lazy_static! {
        static ref pattern: Regex =
            Regex::new(r"pos=<([-\d]+),([-\d]+),([-\d]+)>, r=(\d+)").expect("Parse pattern");
    }
    let captures = pattern.captures(line).expect("Line should match format");
    let location = Point3d::new(
        captures[1].parse().unwrap(),
        captures[2].parse().unwrap(),
        captures[3].parse().unwrap(),
    );
    NanoBot {
        location: location,
        radius: captures[4].parse().unwrap(),
    }
}

struct NanoBot {
    location: Point3d,
    radius: u32,
}
