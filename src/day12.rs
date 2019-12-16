use std::ops::AddAssign;

#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    x: i32,
    y: i32,
    z: i32
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Moon {
    position: Vector3D,
    velocity: Vector3D
}

impl Moon {
    fn total_energy(&self) -> i32 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) *
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

fn next_step(moons: &mut Vec<Moon>) {
    // Update velocities.
    let moons_copy = moons.clone();
    for m1 in moons.iter_mut() {
        for m2 in &moons_copy {
            m1.velocity.x += if m2.position.x > m1.position.x { 1 } else if m2.position.x < m1.position.x { -1 } else { 0 };
            m1.velocity.y += if m2.position.y > m1.position.y { 1 } else if m2.position.y < m1.position.y { -1 } else { 0 };
            m1.velocity.z += if m2.position.z > m1.position.z { 1 } else if m2.position.z < m1.position.z { -1 } else { 0 };
        }
    }

    // Update positions.
    for m in moons.iter_mut() {
        m.position += m.velocity;
    }
}

fn create_moons(moon_positions: &[Vector3D]) -> Vec<Moon> {
    moon_positions.iter().map(|position| Moon { position: *position, velocity: Vector3D { x: 0, y: 0, z: 0 } }).collect()
}

pub fn final_energy(moon_positions: &[Vector3D], steps: u32) -> i32 {
    let mut moons: Vec<Moon> = create_moons(moon_positions);

    for _ in 0 .. steps {
        next_step(&mut moons);
    }

    moons.iter().fold(0, |energy, moon| energy + moon.total_energy())
}

pub fn find_same_state(moon_positions: &[Vector3D]) -> i64 {
    use num::Integer;

    let mut moons: Vec<Moon> = create_moons(moon_positions);
    let initial_state = moons.clone();
    let mut nb_cycles = Vector3D { x: 0, y: 0, z: 0 };

    let mut i = 0;
    loop {
        next_step(&mut moons);
        i += 1;

        if nb_cycles.x == 0 && initial_state.iter().zip(&moons).all(
            |(m1, m2)| m1.position.x == m2.position.x && m1.velocity.x == m2.velocity.x
        ) { nb_cycles.x = i; }

        if nb_cycles.y == 0 && initial_state.iter().zip(&moons).all(
            |(m1, m2)| m1.position.y == m2.position.y && m1.velocity.y == m2.velocity.y
        ) { nb_cycles.y = i; }

        if nb_cycles.z == 0 && initial_state.iter().zip(&moons).all(
            |(m1, m2)| m1.position.z == m2.position.z && m1.velocity.z == m2.velocity.z
        ) { nb_cycles.z = i; }

        if nb_cycles.x != 0 && nb_cycles.y != 0 && nb_cycles.z != 0 {
            return (nb_cycles.x as i64).lcm(&(nb_cycles.y as i64)).lcm(&(nb_cycles.z as i64))
        }
    }
}

pub fn parse_positions(input: &str) -> Vec<Vector3D> {
    use regex::Regex;
    fn build_pattern(var: &str) -> Regex { Regex::new(&format!(r".*{}\W*?=\W*?(-?.*?\d+).*", var)).unwrap() }
    let rex = build_pattern("x");
    let rey = build_pattern("y");
    let rez = build_pattern("z");
    input
        .lines()
        .map(|l|
            Vector3D {
                x: rex.captures(l).unwrap()[1].parse().unwrap(),
                y: rey.captures(l).unwrap()[1].parse().unwrap(),
                z: rez.captures(l).unwrap()[1].parse().unwrap()
            }
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input =
            "<x=-1, y=0, z=2>
             <x=2, y=-10, z=-7>
             <x=4, y=-8, z=8>
             <x=3, y=5, z=-1>";
        let coordinates = parse_positions(input);
        assert_eq!(final_energy(&coordinates, 10), 179);
    }

    #[test]
    fn part2() {
        let input =
            "<x=-8, y=-10, z=0>
             <x=5, y=5, z=10>
             <x=2, y=-7, z=3>
             <x=9, y=-8, z=-3>";
        let coordinates = parse_positions(input);
        assert_eq!(find_same_state(&coordinates), 4_686_774_924);
    }
}