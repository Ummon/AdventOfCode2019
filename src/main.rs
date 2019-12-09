use std::env;
use std::fs;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
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
        "part1: {}, part2: {}",
        day03::manhattan_distance_from_cross_to_port(&day03::split_movements(&movements[0]), &day03::split_movements(&movements[1])),
        day03::first_cross_sum_of_lengths(&day03::split_movements(&movements[0]), &day03::split_movements(&movements[1]))
    )
}

fn day04() -> String {
    format!("")
}

fn day05() -> String {
    let code = common::read_list_of_numbers("data/day05.input", ",");
    format!("part1: {:?}, part2: {:?}", day05::execute_op_code(&mut Vec::from(&code[..]), &[1]), day05::execute_op_code(&mut Vec::from(&code[..]), &[5]))
}

fn day06() -> String {
    let file_content = fs::read_to_string("data/day06.input").unwrap();
    let lines: Vec<&str> = file_content.lines().collect();
    let orbits = day06::build_orbits(&lines);
    format!("part1: {}, part2: {}", day06::total_direct_and_indirect_orbits(&orbits), day06::nb_orbital_transfers(&orbits, "SAN", "YOU"))
}

fn day07() -> String {
    format!("")
}

fn day08() -> String {
    format!("")
}

fn day09() -> String {
    let code = common::read_list_of_numbers("data/day09.input", ",");
    format!("part1: {:?}, part2: {:?}", day09::execute_op_code(&code, &[1]), day09::execute_op_code(&code, &[2]))
}

fn format_micros(t: u128) -> String {
    if t < 10_000 {
        format!("{} μs", t)
    } else if t < 10_000_000u128 {
        format!("{} ms", t / 1_000u128)
    } else {
        format!("{} s", t / 1_000_000u128)
    }
}

fn do_day(days: &[fn() -> String], day: usize) {
    let now = Instant::now();
    println!("Result of day {}: {} (time: {})", day, days[day - 1](), format_micros(now.elapsed().as_micros()));
}

fn main() {
    println!("https://adventofcode.com/2019");

    let days: Vec<fn() -> String> = vec!(
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08,
        day09
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
