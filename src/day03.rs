use std::iter::Iterator;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn split_movements(movements: &str) -> Vec<&str> {
    movements.split(',').collect()
}

fn positions(wire: &[&str]) -> Vec<(i32, i32)> {
    let (mut x, mut y) = (0, 0);
    let mut pos = Vec::<(i32, i32)>::new();
    for mov in wire {
        let distance = mov[1..].parse::<i32>().unwrap();
        match mov.chars().nth(0).unwrap() {
            'U'      => { for y2 in  y+1 ..= y + distance     { pos.push((x, y2)); } y += distance },
            'D'      => { for y2 in (y - distance .. y).rev() { pos.push((x, y2)); } y -= distance },
            'R'      => { for x2 in  x+1 ..= x + distance     { pos.push((x2, y)); } x += distance },
            'L' | _  => { for x2 in (x - distance .. x).rev() { pos.push((x2, y)); } x -= distance },
        }
    }
    pos
}

pub fn manhattan_distance_from_cross_to_port(wire1: &[&str], wire2: &[&str]) -> i32 {
    let positions_wire1: HashSet<(i32, i32)> = HashSet::from_iter(positions(wire1));
    let positions_wire2: HashSet<(i32, i32)> = HashSet::from_iter(positions(wire2));
    let cross: HashSet<_> = positions_wire1.intersection(&positions_wire2).collect();
    cross.iter().map(|(x, y)| x.abs() + y.abs()).min().unwrap()
}

pub fn first_cross_sum_of_lengths(wire1: &[&str], wire2: &[&str]) -> usize {
    let positions_wire1 = positions(wire1);
    let positions_wire1_indexed: HashMap<&(i32, i32), usize> = HashMap::from_iter(positions_wire1.iter().enumerate().map(|(i, pos)| (pos, i)).rev());

    positions(wire2)
        .iter().enumerate().filter_map(
            |(j, pos2)| {
                if let Some (i) = positions_wire1_indexed.get(pos2) {
                    Some (j + i + 2)
                } else {
                    None
                }
            }
        ).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
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

    #[test]
    fn part2() {
        assert_eq!(
            first_cross_sum_of_lengths(
                &split_movements("R8,U5,L5,D3"),
                &split_movements("U7,R6,D4,L5")
            ),
            30
        );

        assert_eq!(
            first_cross_sum_of_lengths(
                &split_movements("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &split_movements("U62,R66,U55,R34,D71,R55,D58,R83")
            ),
            610
        );

        assert_eq!(
            first_cross_sum_of_lengths(
                &split_movements("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &split_movements("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ),
            410
        );
    }
}