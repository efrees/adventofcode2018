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
    time(&day1::solve);
    time(&day2::solve);
    time(&day3::solve);
    time(&day4::solve);
    // time(&day5::solve);
    // time(&day6::solve);
    // time(&day7::solve);
    // time(&day8::solve);
    // time(&day9::solve);
    time(&day10::solve);
    time(&day11::solve);
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
