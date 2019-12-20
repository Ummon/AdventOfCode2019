use super::intcode;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction { Up, Left, Down, Right }

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            '<' => Some(Direction::Left),
            'v' => Some(Direction::Down),
            '>' => Some(Direction::Right),
             _  => None
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Movement { Left, Right }

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct MovementCommand { mov: Movement, steps: u32 }

pub struct RobotTrackingSystem {
    output: Vec<i64>,
    board: Vec<Vec<char>>,
    start_position: (i32, i32),
    start_dir: Direction,
    crossings: Vec<(i32, i32)>,
    dir_commands: Vec<MovementCommand>,
}

impl RobotTrackingSystem {
    fn new() -> Self {
        RobotTrackingSystem {
            output: Vec::new(),
            board: Vec::<Vec<char>>::new(),
            start_position: (0, 0),
            start_dir: Direction::Up,
            crossings: Vec::<(i32, i32)>::new(),
            dir_commands: Vec::<MovementCommand>::new(),
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
                if let Some(dir) =  Direction::from_char(c) {
                    self.start_position = (current_x, self.board.len() as i32);
                    self.start_dir = dir
                }

                current_line.push(c);
                current_x += 1;
            }
        }

        self.output.clear();
        self.run_through();
    }

    // Run the path and find the crossings and define the movements.
    fn run_through(&mut self) {
        let (mut x, mut y) = self.start_position;
        let mut dir = self.start_dir;
        let mut visited_locations = HashSet::<(i32, i32)>::new();
        visited_locations.insert((x, y));

        let mut last_mov = Movement::Left;
        let mut steps = 0;

        'main: loop {
            let positions = [(Direction::Up, (x, y - 1)), (Direction::Left, (x - 1, y)), (Direction::Right, (x + 1, y)), (Direction::Down, (x, y + 1))];

            let next_position = positions.iter().find(|(d, _)| *d == dir).unwrap().1;

            // If the robot can continue straightforward.
            if self.get(next_position.0, next_position.1) == Some('#') {
                if !visited_locations.insert(next_position) {
                    self.crossings.push(next_position);
                }
                steps += 1;
                x = next_position.0;
                y = next_position.1;
                continue;
            }

            if steps != 0 {
                self.dir_commands.push(MovementCommand { mov: last_mov, steps });
                steps = 0;
            }

            for (d, p) in &positions {
                if self.get(p.0, p.1) == Some('#') && !visited_locations.contains(p) {

                    last_mov =
                        match (dir, *d) {
                            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Down) | (Direction::Down, Direction::Left) | (Direction::Left, Direction::Up) => Movement::Right,
                            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Down) | (Direction::Down, Direction::Right) | (Direction::Right, Direction::Up) => Movement::Left,
                            _ => panic!("Unable to find a movement from {:?} to {:?}", dir, *d)
                        };

                    visited_locations.insert(*p);
                    steps += 1;
                    dir = *d;
                    x = p.0;
                    y = p.1;
                    continue 'main;
                }
            }

            break;
        }
    }
}

impl intcode::IO for RobotTrackingSystem {
    // May block.
    fn read(&mut self) -> i64 {
        self.build_board_from_output();
        42 // TODO: part2.
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
    rts.crossings.iter().fold(0, |sum, crossing| sum + crossing.0 * crossing.1)
}

pub fn part2(code: &[i64]) {
    let mut rts = RobotTrackingSystem::new();
    intcode::execute_op_code_with_custom_io(code, &mut rts);

}
