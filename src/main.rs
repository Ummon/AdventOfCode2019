use std::env;

mod day01;
mod common;

fn day01() {
    let masses = common::read_list_of_numbers("data/day01.input");
    println!("Day01 part1: {}", day01::sum_mass_to_fuel(&masses));
    println!("      part2: {}", day01::sum_mass_to_fuel_2(&masses));
}

fn do_day(n: i32) {
    match n {
        1 => day01 (),
        _ => panic!("Unknown day: {}", n)
    }
}

fn main() {
    println!("https://adventofcode.com/2019");

    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().count() == 0 {
        println!("ASD");
    } else {
        for arg in args {
            do_day(arg.parse::<i32>().unwrap());
            //println!("{}", arg);
        }
    }
}
