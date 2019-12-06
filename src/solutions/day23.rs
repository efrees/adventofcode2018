use adventlib;
use adventlib::grid::*;
use regex::Regex;

pub fn solve() {
    println!("Day 23");

    let lines = adventlib::read_input_lines("day23test2input.txt");

    let bots: Vec<_> = lines.iter().map(|l| parse_bot(l)).collect();
    let strongest_bot = bots.iter().max_by_key(|b| b.radius).expect("Must be a max");

    let bots_in_reach = bots_in_reach_of(strongest_bot, &bots);

    println!("Bots reached by strongest: {}", bots_in_reach);

    let origin = Point3d::new(0,0,0);
    let bots_connected = bots.iter().map(|b| reached_by_bot_count(b, &bots));

    let mut max_bot_count = 0;
    let mut max_bot_dist = !0;

    for (i, count) in bots_connected.enumerate() {
        if count >= max_bot_count {
            let bot_dist = bots[i].location.manhattan_dist_to(&origin);

            if count > max_bot_count || bot_dist < max_bot_dist {
                max_bot_count = count;
                max_bot_dist = bot_dist;
            }
        }
    }

    //> 112121559
    println!("Maximally connected count: {}", max_bot_count);
    println!("Distance to maximally connected: {}", max_bot_dist);
}

fn parse_bot(line: &str) -> NanoBot {
    lazy_static! {
        static ref PATTERN: Regex =
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

fn bots_in_reach_of(bot: &NanoBot, all_bots: &Vec<NanoBot>) -> usize {
    all_bots.iter()
        .filter(|b| {
            b.location.manhattan_dist_to(&bot.location) <= bot.radius as i64
        })
        .count()
}

fn reached_by_bot_count(bot: &NanoBot, all_bots: &Vec<NanoBot>) -> usize {
    all_bots.iter()
        .filter(|b| {
            b.location.manhattan_dist_to(&bot.location) <= b.radius as i64
        })
        .count()
}

struct NanoBot {
    location: Point3d,
    radius: u32,
}
