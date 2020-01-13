use super::intcode;
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Range;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction { Up, Left, Down, Right }

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, ()> {
        match c {
            '^' => Ok(Direction::Up),
            '<' => Ok(Direction::Left),
            'v' => Ok(Direction::Down),
            '>' => Ok(Direction::Right),
             _  => Err(())
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Movement { Left, Right }

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct MovementCommand { mov: Movement, steps: u32 }

struct RobotTrackingSystem {
    board: Vec<Vec<char>>,
    start_position: (i32, i32),
    start_dir: Direction,
    dir_commands: Vec<MovementCommand>,
    crossings: Vec<(i32, i32)>,
}

impl RobotTrackingSystem {
    fn new() -> Self {
        RobotTrackingSystem {
            board: Vec::<Vec<char>>::new(),
            start_position: (0, 0),
            start_dir: Direction::Up,
            dir_commands: Vec::<MovementCommand>::new(),
            crossings: Vec::<(i32, i32)>::new(),
        }
    }

    fn from(output: &[i64]) -> Self {
        let mut rts = RobotTrackingSystem::new();
        let mut current_line = Vec::<char>::new();
        let mut current_x = 0;
        for c in output.iter() {
            if *c == 10 {
                rts.board.push(current_line);
                current_line = Vec::<char>::new();
                current_x = 0;
            } else {
                let c = (*c as u8) as char;
                if let Ok(dir) =  Direction::try_from(c) {
                    rts.start_position = (current_x, rts.board.len() as i32);
                    rts.start_dir = dir
                }

                current_line.push(c);
                current_x += 1;
            }
        }
        rts.run_through();
        rts
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < self.board[0].len() as i32 && x >= 0 && y < self.board.len() as i32 && y >= 0 {
            Some(self.board[y as usize][x as usize])
        } else {
            None
        }
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

struct CommandSequences {
    commands: Vec<(usize, Range<usize>)> // Each range is associated with a sequence number (first tuple value).
}

fn is_overlapping<T : PartialOrd>(r1: &Range<T>, r2: &Range<T>) -> bool {
    r1.start < r2.start && r1.end > r2.start || r2.start < r1.start && r2.end > r1.start
}

impl CommandSequences {
    fn new() -> Self {
        CommandSequences {
            commands: Vec::new()
        }
    }

    fn find_sequences(&mut self, movements: &[MovementCommand]) {
        // For each sequence length we try to match them agains the movements.
        let len_min = 3;
        let len_max = 6;

        for l1 in len_min ..= len_max {
            for l2 in len_min ..= len_max {
                for l3 in len_min ..= len_max {
                    self.commands.clear();
                    let mut position: usize = 0;
                    for seq_num in 0 .. 3 {
                        let l = match seq_num { 0 => l1, 1 => l2, _ => l3 };
                        let range = position .. position + l;
                        self.commands.push((seq_num, range.clone()));
                        // Try to find the sequence elsewhere in 'movements'.
                        let mut position2 = position + l;
                        while position2 <= movements.len() - l {
                            let range2 = position2 .. position2 + l;
                            if !self.commands.iter().any(|(_, r)| is_overlapping(&r, &range2)) && movements.get(range.clone()) == movements.get(range2.clone()) {
                                self.commands.push((seq_num, range2));
                                position2 += l;
                            } else {
                                position2 += 1;
                            }
                        }

                        // Update position to the next free position.
                        while self.commands.iter().any(|(_, r)| r.contains(&position)) {
                            position += 1;
                        }
                    }

                    // Check if all movements are included into a sequence.
                    if self.commands.iter().fold(0, |sum, (_, range)| sum + range.len()) == movements.len() {
                        return;
                    }
                }

            }
        }
    }
}

struct Part1 { output: Vec<i64>, }

impl Part1 { fn new() -> Self { Part1 { output: Vec::<i64>::new() } }}

struct Part2 {
    output: Vec<i64>,
    rts: Option<RobotTrackingSystem>,
    commands_sequences: CommandSequences,
    input: Vec<i64>,
    input_position: usize,
    dust_collected: i64,
}

impl Part2 { fn new() -> Self { Part2 { output: Vec::<i64>::new(), rts: None, commands_sequences: CommandSequences::new(), input: Vec::new(), input_position: 0, dust_collected: 0 } } }

impl intcode::IO for Part1 {
    // Read instructions
    fn read(&mut self) -> i64 { 0 }

    // Send to the output channel.
    fn write(&mut self, value: i64) {
        self.output.push(value);
    }
}

impl intcode::IO for Part2 {
    // Read instructions.
    fn read(&mut self) -> i64 {
        if self.rts.is_none() {
            self.rts = Some(RobotTrackingSystem::from(&self.output));
            self.commands_sequences.find_sequences(&self.rts.as_ref().unwrap().dir_commands);

            // 1: add the movements sequences: "A,B,C,A\n" // Max: 10 sequence calls.
            for (i, (seq_num, _)) in self.commands_sequences.commands.iter().sorted_by(|(_, r1), (_, r2)| r1.start.cmp(&r2.start)).enumerate() {
                if i > 0 { self.input.push(44); }
                self.input.push(*seq_num as i64 + 65);
            }
            self.input.push(10);

            // 2: Add the sequence A, B and C: "R,8,L,2,R,1\n", Max: ~6 movements.
            for seq_num in 0 .. 3 {
                let (_, sequence) = self.commands_sequences.commands.iter().find(|(s, _)| *s == seq_num).unwrap();
                for (i, movement_command) in self.rts.as_ref().unwrap().dir_commands.get(sequence.clone()).unwrap().iter().enumerate() {

                    if i > 0 { self.input.push(44); }
                    if movement_command.mov == Movement::Left { self.input.push(76); } else { self.input.push(82); }
                    self.input.push(44);
                    for c in movement_command.steps.to_string().as_bytes() {
                        self.input.push(*c as i64);
                    }
                }
                self.input.push(10);
            }

            // 3: Add "y\n" (continuous video feed activated) or "n\n" (no video).
            self.input.push(110);
            self.input.push(10);
        }

        self.input_position += 1;
        self.input[self.input_position - 1]
    }

    // Send to the output channel.
    fn write(&mut self, value: i64) {
        self.output.push(value);
    }

    fn finished(&mut self) {
        self.dust_collected = *self.output.last().unwrap();
    }
}

pub fn scaffold_intersections(code: &[i64]) -> i32 {
    let mut part1 = Part1::new();
    intcode::execute_op_code_with_custom_io(code, &mut part1);
    let rts = RobotTrackingSystem::from(&part1.output);
    rts.crossings.iter().fold(0, |sum, crossing| sum + crossing.0 * crossing.1)
}

pub fn collected_dust(code: &[i64]) -> i64 {
    let mut part2 = Part2::new();
    intcode::execute_op_code_with_custom_io(code, &mut part2);
    part2.dust_collected
}