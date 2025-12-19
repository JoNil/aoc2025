use std::{any::type_name, fmt::Display, time::Instant};

use aoc2025::*;

fn time<F, N>(f: F, input: &str)
where
    F: FnOnce(&str) -> N,
    N: Display,
{
    let start = Instant::now();
    let answer = f(input);
    let elapsed = start.elapsed();

    println!(
        "{} Time {} us: {answer}",
        type_name::<F>().trim_start_matches("aoc2025::days::"),
        elapsed.as_micros()
    );
}

fn main() {
    let start = Instant::now();

    time(day01::a, day01::INPUT);
    time(day01::b, day01::INPUT);

    time(day02::a, day02::INPUT);
    time(day02::b, day02::INPUT);

    let elapsed = start.elapsed();

    println!("Total Time {} us", elapsed.as_micros());
}
