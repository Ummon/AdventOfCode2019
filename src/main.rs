use std::env;
use std::fs;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod common;

fn day01() -> String {
    let masses = common::read_list_of_numbers("data/day01.input", "\n");
    format!("part1: {}, part2: {}", day01::sum_mass_to_fuel(&masses), day01::sum_mass_to_fuel_2(&masses))
}

fn day02() -> String {
    let code = common::read_list_of_numbers("data/day02.input", ",");
    format!("part1: {}, part2: {}", day02::execute_op_code_with_state_fixed(&mut Vec::from(&code[..])), day02::find_noun_and_verb(&code))
}

fn day03() -> String {
    let file_content = fs::read_to_string("data/day03.input").unwrap();
    let movements: Vec<&str> = file_content.lines().collect();
    format!(
        "part1: {}",
        day03::manhattan_distance_from_cross_to_port(&day03::split_movements(&movements[0]), &day03::split_movements(&movements[1]))
    )
}

fn do_day(days: &[fn() -> String], day: usize) {
    let now = Instant::now();
    println!("Result of day {}: {} (time: {} μs)", day, days[day - 1](), now.elapsed().as_micros());
}

fn main() {
    println!("https://adventofcode.com/2019");

    let days: Vec<fn() -> String> = vec!(
        day01,
        day02,
        day03
    );

    let args: Vec<String> = env::args().skip(1).collect();

    // No argument -> execute all day problems.
    if args.is_empty() {
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
