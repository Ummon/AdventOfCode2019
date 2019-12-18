use super::intcode;
use std::collections::HashSet;

pub fn scaffold_intersections(code: &[i64]) -> i32 {
    let output = intcode::execute_op_code(code, &[]);
    let mut board = Vec::<Vec<char>>::new();
    let mut current_line = Vec::<char>::new();

    let (mut x, mut y) = (0i32, 0i32);
    let mut dir = '^';

    let mut current_x = 0;
    for c in output {
        if c == 10 {
            board.push(current_line);
            current_line = Vec::<char>::new();
            current_x = 0;
            //println!("");
        } else {
            let c = (c as u8) as char;
            if let '^' | '<' | 'v' | '>' = c {
                x = current_x;
                y = board.len() as i32;
                dir = c;
            }
            //print!("{}", c);
            current_line.push(c);
            current_x += 1;
        }
    }

    let get = |x: i32, y: i32| -> Option<char> {
        if x < board[0].len() as i32 && x >= 0 && y < board.len() as i32 && y >= 0 {
            Some(board[y as usize][x as usize])
        } else {
            None
        }
    };

    let mut visited_locations = HashSet::<(i32, i32)>::new();
    let mut crosses = Vec::<(i32, i32)>::new();
    visited_locations.insert((x, y));

    'main: loop {
        let positions = [('^', (x, y - 1)), ('<', (x - 1, y)), ('>', (x + 1, y)), ('v', (x, y + 1))];

        let next_position = positions.iter().find(|(d, _)| *d == dir).unwrap().1;
            //match dir { '^' => positions[0], '<' => positions[1], '>' => positions[2], 'v' | _ => positions[3] };

        // If the robot can continue straightforward.
        if get(next_position.0, next_position.1) == Some('#') {
            if !visited_locations.insert(next_position) {
                crosses.push(next_position);
            }
            x = next_position.0;
            y = next_position.1;
            continue;
        }

        for (d, p) in &positions {
            if get(p.0, p.1) == Some('#') && !visited_locations.contains(p) {
                visited_locations.insert(*p);
                dir = *d;
                x = p.0;
                y = p.1;
                continue 'main;
            }
        }

        break;
    }

    crosses.iter().fold(0, |sum, cross| sum + cross.0 * cross.1)
}