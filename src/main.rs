extern crate adventlib;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::time::Instant;

mod solutions;
use solutions::*;

fn main() {
    time_with_label(&solve_all, "Total time: ");
}

fn solve_all() {
    // time(&day1::solve);
    // time(&day2::solve);
    // time(&day3::solve);
    // time(&day4::solve);
    // time(&day5::solve);
    // time(&day6::solve);
    // time(&day7::solve);
    // time(&day8::solve);
    // time(&day9::solve);
    // time(&day10::solve);
    // time(&day11::solve);
    // time(&day12::solve);
    // time(&day13::solve);
    // time(&day14::solve);
    // time(&day15::solve);
    // time(&day16::solve);
    // time(&day17::solve);
    // time(&day18::solve);
    // time(&day19::solve);
    // time(&day20::solve);
    // time(&day21::solve);
    // time(&day22::solve);
    time(&day23::solve);
}

fn time(f: &Fn()) {
    time_with_label(f, "Solved in");
}

fn time_with_label(f: &Fn(), label: &str) {
    let now = Instant::now();
    f();
    let duration = now.elapsed();
    println!(
        "{} {}.{:09}s\n",
        label,
        duration.as_secs(),
        duration.subsec_nanos()
    );
}
