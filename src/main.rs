use std::env;
use std::time::Instant;

mod day01;
mod common;

fn day01() -> String {
    let masses = common::read_list_of_numbers("data/day01.input");
    format!("part1: {}, part2: {}", day01::sum_mass_to_fuel(&masses), day01::sum_mass_to_fuel_2(&masses))
}

fn do_day(days: &Vec<fn() -> String>, day: usize) {
    let now = Instant::now();
    println!("Result of day {}: {} (time: {} μs)", day, days[day - 1](), now.elapsed().as_micros());
}

fn main() {
    println!("https://adventofcode.com/2019");

    let days: Vec<fn() -> String> = vec!(
        day01
    );

    let args: Vec<String> = env::args().skip(1).collect();

    // No argument -> execute all day problems.
    if args.len() == 0 {
        for i in 1..=days.len() {
            do_day(&days, i)
        }
    } else {
        for arg in args {
            let day = arg.parse::<usize>().unwrap();
            if day > days.len() { panic!("Unknown day: {}", day) }
            do_day(&days, day)
        }
    }
}
