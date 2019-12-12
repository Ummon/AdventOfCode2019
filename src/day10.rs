use std::collections::{HashMap, HashSet};

pub fn read_map(raw: &str) -> Vec<(i32, i32)> {
    let lines: Vec<&str> = raw.lines().map(|l| l.trim()).collect();
    let mut map = Vec::<(i32, i32)>::new();
    for x in 0 .. lines[0].len() {
        for (y, line) in lines.iter().enumerate() {
            if line.chars().nth(x) == Some('#') {
                map.push((x as i32, y as i32));
            }
        }
    }
    map
}

fn angle(x1: i32, y1: i32, x2: i32, y2: i32) -> i64 {
    // Axis are reverted to match the clockwise laser rotation beginning up.
    let angle_f64 = (2.0 * std::f64::consts::PI - ((x1 - x2) as f64).atan2((y1 - y2) as f64)) % (2.0 * std::f64::consts::PI);
    (angle_f64 * 1_000_000.0) as i64
}

fn squared_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i64 {
    ((x1 - x2) as i64).pow(2) + ((y1 - y2) as i64).pow(2)
}

pub fn find_best_location(map: &[(i32, i32)]) -> (usize, (i32, i32)) {
    let mut best_nb_observable_asteroid = 0;
    let (mut best_x, mut best_y) = (0, 0);
    for (x1, y1) in map {
        let mut angles = HashSet::<i64>::new();
        for (x2, y2) in map {
            angles.insert(angle(*x1, *y1, *x2, *y2));
        }
        let n = angles.len();
        if n > best_nb_observable_asteroid {
            best_nb_observable_asteroid = n;
            best_x = *x1;
            best_y = *y1;
        }
    }
    (best_nb_observable_asteroid, (best_x, best_y))
}

type PositionsAndDistances = Vec<((i32, i32), i64)>;

pub fn location_nth_vaporized_asteroid(pos: (i32, i32), map: &[(i32, i32)], n: usize) -> (i32, i32) {
    // Angle -> [(position, distance)].
    let mut asteroids = HashMap::<i64, PositionsAndDistances>::new();

    let (x1, y1) = pos;
    for (x2, y2) in map {
        let angle = angle(x1, y1, *x2 , *y2);
        let dist = squared_distance(x1, y1, *x2 , *y2);
        match asteroids.get_mut(&angle) {
            Some (lineup_asteroids) => lineup_asteroids.push(((*x2, *y2), dist)),
            None => { asteroids.insert(angle, vec![((*x2, *y2), dist)]); }
        }
    }

    // Sort everything by angle and by distance.
    let mut sorted_asteroids: Vec<(&i64, &mut PositionsAndDistances)> = asteroids.iter_mut().collect();
    sorted_asteroids.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));
    for (_, lineup_asteroids) in sorted_asteroids.iter_mut() {
        lineup_asteroids.sort_by(|(_, l1), (_, l2)| l1.cmp(l2))
    }

    let mut i = 1;
    loop {
        for (_, lineup_asteroids) in sorted_asteroids.iter_mut() {
            let ((x, y), _) = lineup_asteroids.remove(0);
            if i == n { return (x, y) }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_1() {
        let raw_map =
            ".#..#
             .....
             #####
             ....#
             ...##";
        let map = read_map(raw_map);
        assert_eq!(find_best_location(&map).0, 8);
    }

    #[test]
    fn part1_sample_2() {
        let raw_map =
            "......#.#.
             #..#.#....
             ..#######.
             .#.#.###..
             .#..#.....
             ..#....#.#
             #..#....#.
             .##.#..###
             ##...#..#.
             .#....####";
        let map = read_map(raw_map);
        assert_eq!(find_best_location(&map).0, 33);
    }

    #[test]
    fn part1_sampl3() {
        let raw_map =
            "#.#...#.#.
             .###....#.
             .#....#...
             ##.#.#.#.#
             ....#.#.#.
             .##..###.#
             ..#...##..
             ..##....##
             ......#...
             .####.###.";
        let map = read_map(raw_map);
        assert_eq!(find_best_location(&map).0, 35);
    }

    #[test]
    fn part1_sample_4() {
        let raw_map =
            ".#..#..###
             ####.###.#
             ....###.#.
             ..###.##.#
             ##.##.#.#.
             ....###..#
             ..#.#..#.#
             #..#.#.###
             .##...##.#
             .....#.#..";
        let map = read_map(raw_map);
        assert_eq!(find_best_location(&map).0, 41);
    }

    #[test]
    fn part1_sample_5() {
        let raw_map =
            ".#..##.###...#######
             ##.############..##.
             .#.######.########.#
             .###.#######.####.#.
             #####.##.#.##.###.##
             ..#####..#.#########
             ####################
             #.####....###.#.#.##
             ##.#################
             #####.##.###..####..
             ..######..##.#######
             ####.##.####...##..#
             .#####..#.######.###
             ##...#.##########...
             #.##########.#######
             .####.#.###.###.#.##
             ....##.##.###..#####
             .#.#.###########.###
             #.#.#.#####.####.###
             ###.##.####.##.#..##";
        let map = read_map(raw_map);
        assert_eq!(find_best_location(&map).0, 210);
    }


    #[test]
    fn part2_sample_1() {
        let raw_map =
            ".#....#####...#..
             ##...##.#####..##
             ##...#...#.#####.
             ..#.....X...###..
             ..#.#.....#....##";
        let map = read_map(raw_map);
        let pos = (8, 3);
        let pos_9th = location_nth_vaporized_asteroid(pos, &map, 9);
        assert_eq!(pos_9th, (15, 1));

        let pos_18th = location_nth_vaporized_asteroid(pos, &map, 18);
        assert_eq!(pos_18th, (4, 4));
    }

    #[test]
    fn part2_sample_2() {
        let raw_map =
            ".#..##.###...#######
             ##.############..##.
             .#.######.########.#
             .###.#######.####.#.
             #####.##.#.##.###.##
             ..#####..#.#########
             ####################
             #.####....###.#.#.##
             ##.#################
             #####.##.###..####..
             ..######..##.#######
             ####.##.####...##..#
             .#####..#.######.###
             ##...#.##########...
             #.##########.#######
             .####.#.###.###.#.##
             ....##.##.###..#####
             .#.#.###########.###
             #.#.#.#####.####.###
             ###.##.####.##.#..##";
        let map = read_map(raw_map);
        let pos = find_best_location(&map).1;
        let pos_200th = location_nth_vaporized_asteroid(pos, &map, 200);
        assert_eq!(pos_200th, (8, 2));
    }
}