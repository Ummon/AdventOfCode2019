pub fn sum_mass_to_fuel(masses: &[i32]) -> i32 {
    masses.iter().fold(0, |sum, mass| sum + mass_to_fuel(*mass))
}

pub fn sum_mass_to_fuel_2(masses: &[i32]) -> i32 {
    masses.iter().fold(0, |sum, mass| sum + mass_to_fuel_2(*mass))
}

fn mass_to_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn mass_to_fuel_2(mass: i32) -> i32 {
    let mut sum = 0;
    let mut current_mass = mass;
    loop {
        let fuel = mass_to_fuel(current_mass);
        if fuel <= 0 { break }
        current_mass = fuel;
        sum += fuel;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cases() {
        assert_eq!(mass_to_fuel(12), 2);
        assert_eq!(mass_to_fuel(14), 2);
        assert_eq!(mass_to_fuel(1969), 654);
        assert_eq!(mass_to_fuel(100756), 33583);
    }

    #[test]
    fn simple_cases_2() {
        assert_eq!(mass_to_fuel_2(14), 2);
        assert_eq!(mass_to_fuel_2(1969), 966);
        assert_eq!(mass_to_fuel_2(100756), 50346);
    }
}