use super::intcode;
use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum LocationState {
    Unknown,
    Wall,
    Visited,
    DeadEnd,
}

struct DroidTrackingSystem {
    board: HashMap<(i32, i32), LocationState>,
    current_path: Vec<(i32, i32)>,
    oxygen_found: bool
}

impl DroidTrackingSystem {
    fn new() -> DroidTrackingSystem {
        DroidTrackingSystem {
            board: HashMap::from_iter(vec![((0, 0), LocationState::Visited)].into_iter()),
            current_path: vec![(0, 0)],
            oxygen_found: false
        }
    }

    fn current_position(&self) -> (i32, i32) { *self.current_path.last().unwrap() }

    fn positions_around(&self) -> Vec<(i64, (i32, i32))> {
        let (x, y) = self.current_position();
        vec![(1, (x, y + 1)), (2, (x, y - 1)), (3, (x - 1, y)), (4, (x + 1, y))]
    }

    fn get_state(&self, position: (i32, i32)) -> LocationState {
        match self.board.get(&position) {
            Some(state) => *state,
            None => LocationState::Unknown
        }
    }

    fn set_state(&mut self, position: (i32, i32), state: LocationState) {
        self.board.insert(position, state);
    }

    // 1: north, 2: south, 3: west, 4: est.
    fn next_movement(&mut self) -> i64 {
        for (mov, pos) in self.positions_around() {
            if self.get_state(pos) == LocationState::Unknown {
                self.current_path.push(pos);
                return mov
            }
        }

        for (mov, pos) in self.positions_around() {
            if self.get_state(pos) == LocationState::Visited {
                self.set_state(self.current_position(), LocationState::DeadEnd);
                self.current_path.pop();
                return mov
            }
        }

        panic!("Droid stuck");
    }

    // 0: droid hit a wall, 1: droid moved one step, 2: droid moved one step and has found the oxygen system.
    fn reply_from_droid(&mut self, status: i64) {
        match status {
            0 => {
                self.set_state(self.current_position(), LocationState::Wall);
                self.current_path.pop();
            }
            1 => self.set_state(self.current_position(), LocationState::Visited),
            2 => self.oxygen_found = true,
            _ => panic!("Unkown droid status: {}", status)
        }
    }
}

impl intcode::IO for DroidTrackingSystem {
    // May block.
    fn read(&mut self) -> i64 {
        self.next_movement()
    }

    // Send to the output channel.
    fn write(&mut self, value: i64) {
        self.reply_from_droid(value);
    }

    fn halt(&self) -> bool {
        self.oxygen_found
    }
}

pub fn nb_of_movement_to_reach_oxygen(code: &[i64]) -> i32 {
    let mut dts = DroidTrackingSystem::new();
    intcode::execute_op_code_with_custom_io(code, &mut dts);
    dts.current_path.len() as i32 - 1
}