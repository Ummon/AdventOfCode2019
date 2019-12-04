use std::iter::Iterator;
use std::collections::HashSet;

pub fn split_movements(movements: &str) -> Vec<&str> {
    movements.split(',').collect()
}

pub fn manhattan_distance_from_cross_to_port(wire1: &[&str], wire2: &[&str]) -> i32 {
    fn positions(wire: &[&str]) -> HashSet<(i32, i32)> {
        let (mut x, mut y) = (0, 0);
        let mut pos = HashSet::<(i32, i32)>::new();
        for mov in wire {
            let distance = mov[1..].parse::<i32>().unwrap();
            match mov.chars().nth(0).unwrap() {
                'U'      => { for y2 in y+1 ..= y + distance { pos.insert((x, y2)); } y += distance },
                'D'      => { for y2 in y - distance .. y    { pos.insert((x, y2)); } y -= distance },
                'R'      => { for x2 in x+1 ..= x + distance { pos.insert((x2, y)); } x += distance },
                'L' | _  => { for x2 in x - distance .. x    { pos.insert((x2, y)); } x -= distance },
            }
        }
        pos
    }

    let (positions_wire1, positions_wire2) = (positions(wire1), positions(wire2));
    let cross: HashSet<_> = positions_wire1.intersection(&positions_wire2).collect();
    cross.iter().map(|(x, y)| x.abs() + y.abs()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cases() {
        assert_eq!(
            manhattan_distance_from_cross_to_port(
                &split_movements("R8,U5,L5,D3"),
                &split_movements("U7,R6,D4,L4")
            ),
            6
        );

        assert_eq!(
            manhattan_distance_from_cross_to_port(
                &split_movements("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &split_movements("U62,R66,U55,R34,D71,R55,D58,R83")
            ),
            159
        );

        assert_eq!(
            manhattan_distance_from_cross_to_port(
                &split_movements("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &split_movements("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ),
            135
        );
    }
}