use std::collections::HashMap;

#[derive(Debug)]
pub struct Chemical {
    quantity: i64,
    name: String,
}

type Reactions = HashMap<String, (i64, Vec<Chemical>)>;

fn parse_chemical(input: &str) -> Chemical {
    let quantity_and_name: Vec<&str> = input.trim().split(' ').collect();
    Chemical {
        quantity: quantity_and_name[0].parse::<i64>().unwrap(),
        name: String::from(quantity_and_name[1].trim()),
    }
}

pub fn parse(input: &str) -> Reactions {
    let mut result = Reactions::new();
    for line in input.lines() {
        let reaction: Vec<&str> = line.split("=>").collect();
        let input_chemicals: Vec<Chemical> = reaction[0].split(',').map(parse_chemical).collect();
        let output_chemical = parse_chemical(reaction[1]);
        result.insert(output_chemical.name, (output_chemical.quantity, input_chemicals));
    }
    result
}

pub fn ore_needed_per_fuel(reactions: &Reactions) -> i64 {
    let mut remainders = HashMap::new();
    ore_needed(reactions, 1, &mut remainders)
}

fn ore_needed(reactions: &Reactions, fuel_quantity: i64, remainders: &mut HashMap<String, i64>) -> i64 {

    fn needed(reactions: &Reactions, chemicals: &[Chemical], remainders: &mut HashMap<String, i64>) -> i64 {
        chemicals.iter().fold(0, |sum, chemical| {
            let quantity_needed =
                match remainders.get(&chemical.name) {
                    Some(quantity) => {
                        let remainder = quantity - chemical.quantity;
                        if remainder < 0  {
                            remainders.remove(&chemical.name);
                            remainder.abs()
                        } else {
                            remainders.insert(chemical.name.clone(), remainder);
                            0
                        }
                    },
                    None => chemical.quantity
                };

            sum +
                if chemical.name == "ORE" {
                    quantity_needed
                } else {
                    match reactions.get(&chemical.name) {
                        Some((quantity_produced, chemicals_needed)) => {
                            let n = (quantity_needed / quantity_produced) + (if quantity_needed % quantity_produced == 0 { 0 } else { 1 });
                            let rem = n * quantity_produced - quantity_needed;
                            match remainders.get(&chemical.name) {
                                Some(&q) => { remainders.insert(chemical.name.clone(), q + rem); },
                                None => { remainders.insert(chemical.name.clone(), rem); }
                            }
                            needed(reactions, &chemicals_needed.iter().map(|c| Chemical { quantity: n * c.quantity, name: c.name.clone() }).collect::<Vec<Chemical>>(), remainders)
                        },
                        None => panic!("Unable to find reaction to create {}", chemical.name)
                    }
                }
        })
    }

    needed(reactions, &[Chemical { quantity: fuel_quantity, name: String::from("FUEL") }], remainders)
}

pub fn fuel_produced(reactions: &Reactions, ore: i64, ore_per_fuel: i64) -> i64 {
    let mut ore_available = ore;
    let mut fuel_produced = 0;
    let mut remainders = HashMap::new();

    loop {
        let fuel = 1.max(ore_available / ore_per_fuel); // Approximate the fuel we can produce.
        ore_available -= ore_needed(reactions, fuel, &mut remainders);

        if ore_available <= 0 {
            return fuel_produced
        }

        fuel_produced += fuel;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_1() {
        let input =
            "10 ORE => 10 A
             1 ORE => 1 B
             7 A, 1 B => 1 C
             7 A, 1 C => 1 D
             7 A, 1 D => 1 E
             7 A, 1 E => 1 FUEL";
        let reactions = parse(input);
        assert_eq!(ore_needed_per_fuel(&reactions), 31);
    }

    #[test]
    fn part1_sample_2() {
        let input =
            "9 ORE => 2 A
             8 ORE => 3 B
             7 ORE => 5 C
             3 A, 4 B => 1 AB
             5 B, 7 C => 1 BC
             4 C, 1 A => 1 CA
             2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = parse(input);
        assert_eq!(ore_needed_per_fuel(&reactions), 165);
    }

    #[test]
    fn part1_sample_3() {
        let input =
            "157 ORE => 5 NZVS
             165 ORE => 6 DCFZ
             44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
             12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
             179 ORE => 7 PSHF
             177 ORE => 5 HKGWZ
             7 DCFZ, 7 PSHF => 2 XJWVT
             165 ORE => 2 GPVTF
             3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let reactions = parse(input);
        assert_eq!(ore_needed_per_fuel(&reactions), 13_312);
    }

    #[test]
    fn part1_sample_4() {
        let input =
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
             17 NVRVD, 3 JNWZP => 8 VPVL
             53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
             22 VJHF, 37 MNCFX => 5 FWMGM
             139 ORE => 4 NVRVD
             144 ORE => 7 JNWZP
             5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
             5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
             145 ORE => 6 MNCFX
             1 NVRVD => 8 CXFTF
             1 VJHF, 6 MNCFX => 4 RFSQX
             176 ORE => 6 VJHF";
        let reactions = parse(input);
        assert_eq!(ore_needed_per_fuel(&reactions), 180_697);
    }

    #[test]
    fn part1_sample_5() {
        let input =
            "171 ORE => 8 CNZTR
             7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
             114 ORE => 4 BHXH
             14 VRPVC => 6 BMBT
             6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
             6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
             15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
             13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
             5 BMBT => 4 WPTQ
             189 ORE => 9 KTJDG
             1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
             12 VRPVC, 27 CNZTR => 2 XDBXC
             15 KTJDG, 12 BHXH => 5 XCVML
             3 BHXH, 2 VRPVC => 7 MZWV
             121 ORE => 7 VRPVC
             7 XCVML => 6 RJRHP
             5 BHXH, 4 VRPVC => 5 LTCX";
        let reactions = parse(input);
        assert_eq!(ore_needed_per_fuel(&reactions), 2_210_736);
    }

    #[test]
    fn part2_sample_1() {
        let input =
            "157 ORE => 5 NZVS
             165 ORE => 6 DCFZ
             44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
             12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
             179 ORE => 7 PSHF
             177 ORE => 5 HKGWZ
             7 DCFZ, 7 PSHF => 2 XJWVT
             165 ORE => 2 GPVTF
             3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let reactions = parse(input);
        let ore_per_fuel = ore_needed_per_fuel(&reactions);
        assert_eq!(fuel_produced(&reactions, 1_000_000_000_000, ore_per_fuel), 82_892_753);
    }

    #[test]
    fn part2_sample_2() {
        let input =
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
             17 NVRVD, 3 JNWZP => 8 VPVL
             53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
             22 VJHF, 37 MNCFX => 5 FWMGM
             139 ORE => 4 NVRVD
             144 ORE => 7 JNWZP
             5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
             5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
             145 ORE => 6 MNCFX
             1 NVRVD => 8 CXFTF
             1 VJHF, 6 MNCFX => 4 RFSQX
             176 ORE => 6 VJHF";
        let reactions = parse(input);
        let ore_per_fuel = ore_needed_per_fuel(&reactions);
        assert_eq!(fuel_produced(&reactions, 1_000_000_000_000, ore_per_fuel), 5_586_022);
    }

    #[test]
    fn part2_sample_3() {
        let input =
            "171 ORE => 8 CNZTR
             7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
             114 ORE => 4 BHXH
             14 VRPVC => 6 BMBT
             6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
             6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
             15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
             13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
             5 BMBT => 4 WPTQ
             189 ORE => 9 KTJDG
             1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
             12 VRPVC, 27 CNZTR => 2 XDBXC
             15 KTJDG, 12 BHXH => 5 XCVML
             3 BHXH, 2 VRPVC => 7 MZWV
             121 ORE => 7 VRPVC
             7 XCVML => 6 RJRHP
             5 BHXH, 4 VRPVC => 5 LTCX";
        let reactions = parse(input);
        let ore_per_fuel = ore_needed_per_fuel(&reactions);
        assert_eq!(fuel_produced(&reactions, 1_000_000_000_000, ore_per_fuel), 460_664);
    }
}