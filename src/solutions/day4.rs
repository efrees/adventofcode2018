use chrono::{DateTime, TimeZone, Timelike, Utc};
use regex::Regex;
use std::collections::*;

#[derive(Debug, PartialEq)]
enum GuardLogType {
    GuardStart,
    GuardSleep,
    GuardWake,
}

#[derive(Debug)]
struct GuardLog {
    time: DateTime<Utc>,
    log_type: GuardLogType,
    guard: u32,
}

pub fn solve() {
    println!("Day 4");

    let lines = adventlib::read_input_lines("day4input.txt");

    let mut guard_logs: Vec<_> = lines.iter().map(|x| parse_log_from_line(&x)).collect();

    guard_logs.sort_by_key(|x| x.time);

    let guard_awake_value = 999;
    let mut current_guard = 0;
    let mut guard_asleep_since = guard_awake_value;
    let mut guard_minutes = HashMap::<u32, Vec<i32>>::new();
    for log in guard_logs.iter() {
        if !guard_minutes.contains_key(&log.guard) {
            let initial_minutes: Vec<_> = vec![0; 60];
            guard_minutes.insert(log.guard, initial_minutes);
        }

        // Log the minutes for the previous sleeping guard
        if guard_asleep_since != guard_awake_value {
            let mut end_time = 60;
            if log.log_type == GuardLogType::GuardWake {
                // Wake will always apply to current_guard
                end_time = log.time.minute();
            }

            for minute in guard_asleep_since..end_time {
                guard_minutes
                    .entry(current_guard)
                    .and_modify(|array| array[minute as usize] += 1);
            }
        }

        match &log.log_type {
            GuardLogType::GuardStart => {
                current_guard = log.guard;
                guard_asleep_since = guard_awake_value;
            }
            GuardLogType::GuardWake => guard_asleep_since = guard_awake_value,
            GuardLogType::GuardSleep => guard_asleep_since = log.time.minute(),
        }
    }

    guard_minutes.remove(&0);

    assert_eq!(guard_asleep_since, guard_awake_value);

    let mut max_guard_total = 0;
    let mut guard_with_max_total: u32 = 0;
    let mut max_minute_for_guard: u32 = 999;

    let mut overall_max = 0;
    let mut guard_with_overall_max: u32 = 0;
    let mut overall_max_minute: u32 = 999;

    for guard in guard_minutes.keys() {
        let guard_array = guard_minutes.get(&guard).unwrap();
        let guard_total = guard_array.iter().sum();
        let max_minute: u32 = guard_array.iter().zip(0..=59).max().unwrap().1;
        if guard_total > max_guard_total {
            max_guard_total = guard_total;
            guard_with_max_total = *guard;
            max_minute_for_guard = max_minute;
        }

        if guard_array[max_minute as usize] > overall_max {
            overall_max = guard_array[max_minute as usize];
            overall_max_minute = max_minute;
            guard_with_overall_max = *guard;
        }
    }

    println!(
        "Answer (strategy 1): {}",
        guard_with_max_total * max_minute_for_guard
    );
    println!(
        "Answer (strategy 2): {}",
        guard_with_overall_max * overall_max_minute
    );
}

fn parse_log_from_line(line: &str) -> GuardLog {
    lazy_static! {
        static ref log_pattern: Regex = Regex::new(r"^\[(.*?)\] (.*)$").unwrap();
    }
    let pattern_captures = log_pattern.captures_iter(line).next().unwrap();
    let log_info = parse_log_type_from_message(&pattern_captures[2]);
    let time = Utc
        .datetime_from_str(&pattern_captures[1], "%Y-%m-%d %H:%M")
        .expect("Date parsing");
    return GuardLog {
        time: time,
        log_type: log_info.0,
        guard: log_info.1,
    };
}

fn parse_log_type_from_message(message: &str) -> (GuardLogType, u32) {
    if message.contains("begins") {
        lazy_static! {
            static ref message_pattern: Regex = Regex::new(r"Guard #(\d+) begins").unwrap();
        }
        let pattern_captures = message_pattern.captures_iter(message).next().unwrap();
        let guard_id = pattern_captures[1].to_string().parse::<u32>().unwrap();
        return (GuardLogType::GuardStart, guard_id);
    } else if message.contains("asleep") {
        return (GuardLogType::GuardSleep, 0);
    } else {
        return (GuardLogType::GuardWake, 0);
    }
}
