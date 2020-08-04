use std::env;
use std::fs;
use std::time::Instant;

mod common;
mod intcode;
mod day01;
mod day02;
mod day03;
mod day04;
mod day06;
mod day07;
mod day08;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

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
    let raw = fs::read_to_string("data/day04.input").unwrap();
    let (min, max) = day04::parse_range(&raw);
    format!("part1: {:?}, part2: {}", day04::nb_passwords_part1(min, max), day04::nb_passwords_part2(min, max))
}

fn day05() -> String {
    let code = common::read_list_of_numbers("data/day05.input", ",");
    format!("part1: {:?}, part2: {:?}", intcode::execute_op_code(&code, &[1]), intcode::execute_op_code(&code, &[5]))
}

fn day06() -> String {
    let file_content = fs::read_to_string("data/day06.input").unwrap();
    let lines: Vec<&str> = file_content.lines().collect();
    let orbits = day06::build_orbits(&lines);
    format!("part1: {}, part2: {}", day06::total_direct_and_indirect_orbits(&orbits), day06::nb_orbital_transfers(&orbits, "SAN", "YOU"))
}

fn day07() -> String {
    let code = common::read_list_of_numbers("data/day07.input", ",");

    format!("part1: {}, part2: {}", day07::find_largest_last_thruster_signal(&code), day07::find_largest_last_thruster_signal_with_feedback_loop(&code))
}

fn day08() -> String {
    let img = fs::read_to_string("data/day08.input").unwrap();

    let raw = day08::read_from_string(&img);
    let layers = day08::decode_image(&raw, 25, 6);

    let layer = day08::layer_with_fewer_0(&layers[..]);
    let merged = day08::merge_layers(&layers[..]);

    format!("part1: {}, part2:\n{}", day08::one_digits_times_two_digits(layer), common::layer_to_printable_string(&merged, 25))
}

fn day09() -> String {
    let code = common::read_list_of_numbers::<&str, i64>("data/day09.input", ",");

    format!("part1: {:?}, part2: {:?}", intcode::execute_op_code(&code, &[1]), intcode::execute_op_code(&code, &[2]))
}

fn day10() -> String {
    let map = day10::read_map(&fs::read_to_string("data/day10.input").unwrap());
    let (n, location) = day10::find_best_location(&map);
    let (x, y) = day10::location_nth_vaporized_asteroid(location, &map, 200);
    format!("part1: {}, part2: {}", n, x * 100 + y)
}

fn day11() -> String {
    let code = common::read_list_of_numbers::<&str, i64>("data/day11.input", ",");
    let panels = day11::run_robot(&code, 1);
    let (layer, width) = day11::panels_to_layer(&panels);

    format!("part1: {:?}, part2:\n{}", day11::run_robot(&code, 0).len(), common::layer_to_printable_string(&layer, width))
}

fn day12() -> String {
    let coordinates = day12::parse_positions(&fs::read_to_string("data/day12.input").unwrap());
    format!("part1: {}, part2: {}", day12::final_energy(&coordinates, 1000), day12::find_same_state(&coordinates))
}

fn day13() -> String {
    let code = common::read_list_of_numbers::<&str, i64>("data/day13.input", ",");
    let mut modified_code = Vec::from(&code[..]);
    modified_code[0] = 2;
    format!("part1: {}, part2: {}", day13::count_nb_block(&code), day13::final_score(&modified_code))
}

fn day14() -> String {
    let reactions = day14::parse(&fs::read_to_string("data/day14.input").unwrap());

    let ore_per_fuel = day14::ore_needed_per_fuel(&reactions);
    format!("part1: {}, part2: {}", ore_per_fuel, day14::fuel_produced(&reactions, 1_000_000_000_000, ore_per_fuel))
}

fn day15() -> String {
    let code = common::read_list_of_numbers("data/day15.input", ",");
    let (n, dts) = day15::nb_of_movement_to_reach_oxygen(&code);
    format!("part1: {}, part2: {}", n, day15::time_to_flood_the_area(&dts))
}

fn day16() -> String {
    let signal_raw = fs::read_to_string("data/day16.input").unwrap();
    let signal = day16::parse(&signal_raw);
    let output_part_1 = day16::fft(&signal, &[0, 1, 0, -1], 100, 0, 8, 1);
    // let output_part_2 = day16::part2(&signal);
    format!("part1: {}, part2: {}", day16::digits_as_string(&output_part_1), /*day16::digits_as_string(&output_part_2)*/ "<skipped: take too long ~ 1 min>")
}

fn day17() -> String {
    let mut code = common::read_list_of_numbers("data/day17.input", ",");
    let intersections = day17::scaffold_intersections(&code);
    code[0] = 2;
    let dust = day17::collected_dust(&code);
    format!("part1: {}, part2: {}", intersections, dust)
}

fn day18() -> String {
    let vault_raw = fs::read_to_string("data/day18.input").unwrap();
    let vault = day18::Vault::parse(&vault_raw);
    let nb_steps = day18::nb_steps_to_collect_all_key(&vault);

    format!("part1: {}, part2: {}", nb_steps, "")
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
    println!("Result of day {:02}: {} (time: {})", day, days[day - 1](), format_micros(now.elapsed().as_micros()));
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
        day09,
        day10,
        day11,
        day12,
        day13,
        day14,
        day15,
        day16,
        day17,
        day18,
    );

    let args: Vec<String> = env::args().skip(1).collect();

    // No argument -> execute all day problems.
    if args.is_empty() {
        let now = Instant::now();
        for i in 1 ..= days.len() {
            do_day(&days, i)
        }
        println!("Time to execute all days: {}", format_micros(now.elapsed().as_micros()));
    } else {
        for arg in args {
            match arg.parse::<usize>() {
                Ok(day) if day >= 1 && day <= days.len() =>
                    do_day(&days, day),
                Ok(day) =>
                    println!("Unknown day: {}", day),
                Err(error) =>
                    println!("Unable to parse day number: \"{}\", error: {}", arg, error)
            }
        }
    }
}
