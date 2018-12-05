extern crate chrono;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    time(&day1::solve);
    time(&day2::solve);
    time(&day3::solve);
    time(&day4::solve);
}

fn time(f: &Fn()) {
    let now = Instant::now();
    f();
    let duration = now.elapsed();
    println!(
        "Solved in {}.{:09}s",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}
