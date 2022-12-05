use std::{cmp, collections::HashMap};

// All planets indexing their parent (planet -> parent).
type Orbits = HashMap<String, String>;

pub fn build_orbits(orbits_str: &[&str]) -> Orbits {
    let mut orbits = Orbits::new();
    for orbit in orbits_str {
        let planets: Vec<&str> = orbit.trim().split(')').collect();
        orbits.insert(String::from(planets[1]), String::from(planets[0]));
    }
    orbits
}

fn parents<'a>(orbits: &'a Orbits, planet: &str) -> Vec<&'a str> {
    let mut parents = Vec::<&str>::new();
    let mut current_planet = planet;

    while let Some (parent) = orbits.get(current_planet) {
        parents.insert(0, parent);
        current_planet = parent;
    }

    parents
}

pub fn total_direct_and_indirect_orbits(orbits: &Orbits) -> usize {
    orbits.keys().fold(0, |sum, planet| { sum + parents(orbits, &planet).len() })
}

pub fn nb_orbital_transfers(orbits: &Orbits, loc1: &str, loc2: &str) -> usize {
    let parents_loc1 = parents(orbits, loc1);
    let parents_loc2 = parents(orbits, loc2);
    for i in 0..cmp::min(parents_loc1.len(), parents_loc2.len()) {
        if parents_loc1[i] != parents_loc2[i] {
            return parents_loc1.len() + parents_loc2.len() - 2 * i
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let lines: Vec<&str> =
            "COM)B
                B)C
                C)D
                D)E
                E)F
                B)G
                G)H
                D)I
                E)J
                J)K
                K)L".lines().collect();

        let n = total_direct_and_indirect_orbits(&build_orbits(&lines));
        assert_eq!(n, 42);
    }

    #[test]
    fn part2() {
        let lines: Vec<&str> =
            "COM)B
                B)C
                C)D
                D)E
                E)F
                B)G
                G)H
                D)I
                E)J
                J)K
                K)L
                K)YOU
                I)SAN".lines().collect();

        let n = nb_orbital_transfers(&build_orbits(&lines), "SAN", "YOU");
        assert_eq!(n, 4);
    }
}