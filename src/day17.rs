use super::intcode;
use std::collections::HashSet;

pub struct RobotTrackingSystem {
    output: Vec<i64>,
    board: Vec<Vec<char>>,
    start_position: (i32, i32),
    start_dir: char,
}

impl RobotTrackingSystem {
    fn new() -> Self {
        RobotTrackingSystem {
            output: Vec::new(),
            board: Vec::<Vec<char>>::new(),
            start_position: (0, 0),
            start_dir: '^',
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < self.board[0].len() as i32 && x >= 0 && y < self.board.len() as i32 && y >= 0 {
            Some(self.board[y as usize][x as usize])
        } else {
            None
        }
    }

    fn build_board_from_output(&mut self) {
        // If the board has already been read.
        if !self.board.is_empty() {
            return;
        }

        let mut current_line = Vec::<char>::new();
        let mut current_x = 0;
        for c in self.output.iter() {
            if *c == 10 {
                self.board.push(current_line);
                current_line = Vec::<char>::new();
                current_x = 0;
            } else {
                let c = (*c as u8) as char;
                if let '^' | '<' | 'v' | '>' = c {
                    self.start_position = (current_x, self.board.len() as i32);
                    self.start_dir = c;
                }
                current_line.push(c);
                current_x += 1;
            }
        }
    }
}

impl intcode::IO for RobotTrackingSystem {
    // May block.
    fn read(&mut self) -> i64 {
        self.build_board_from_output();
        42
    }

    // Send to the output channel.
    fn write(&mut self, value: i64) {
        self.output.push(value);
    }

    fn finished(&mut self) {
        self.build_board_from_output();
    }
}

pub fn scaffold_intersections(code: &[i64]) -> i32 {
    let mut rts = RobotTrackingSystem::new();
    intcode::execute_op_code_with_custom_io(code, &mut rts);

    let (mut x, mut y) = rts.start_position;
    let mut dir = rts.start_dir;
    let mut visited_locations = HashSet::<(i32, i32)>::new();
    let mut crosses = Vec::<(i32, i32)>::new();
    visited_locations.insert((x, y));

    'main: loop {
        let positions = [('^', (x, y - 1)), ('<', (x - 1, y)), ('>', (x + 1, y)), ('v', (x, y + 1))];

        let next_position = positions.iter().find(|(d, _)| *d == dir).unwrap().1;

        // If the robot can continue straightforward.
        if rts.get(next_position.0, next_position.1) == Some('#') {
            if !visited_locations.insert(next_position) {
                crosses.push(next_position);
            }
            x = next_position.0;
            y = next_position.1;
            continue;
        }

        for (d, p) in &positions {
            if rts.get(p.0, p.1) == Some('#') && !visited_locations.contains(p) {
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

pub fn part2(code: &[i64]) {

}