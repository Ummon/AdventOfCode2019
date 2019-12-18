use super::intcode;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum LocationState {
    Unknown,
    Wall,
    Visited,
    DeadEnd,
}

#[derive(Clone)]
pub struct DroidTrackingSystem {
    board: HashMap<(i32, i32), LocationState>,
    current_path: Vec<(i32, i32)>,
    oxygen_location: (i32, i32),
    steps_to_oxygen: i32,
    all_locations_explored: bool,
}

impl DroidTrackingSystem {
    fn new() -> Self {
        DroidTrackingSystem {
            board: HashMap::from_iter(vec![((0, 0), LocationState::Visited)].into_iter()),
            current_path: vec![(0, 0)],
            oxygen_location: (0, 0),
            steps_to_oxygen: 0,
            all_locations_explored: false
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

        self.all_locations_explored = true;
        1
    }

    // 0: droid hit a wall, 1: droid moved one step, 2: droid moved one step and has found the oxygen system.
    fn reply_from_droid(&mut self, status: i64) {
        if status == 0 {
            self.set_state(self.current_position(), LocationState::Wall);
            self.current_path.pop();
        } else if status == 1 || status == 2 {
            self.set_state(self.current_position(), LocationState::Visited);

            // We need to explore all positions even if we find the oxygen to compute the time (see 'time_to_flood_the_area') in part 2.
            if status == 2 {
                self.steps_to_oxygen = self.current_path.len() as i32 - 1;
                self.oxygen_location = self.current_position();
            }
        } else {
            panic!("Unkown droid status: {}", status)
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
        self.all_locations_explored
    }
}

pub fn nb_of_movement_to_reach_oxygen(code: &[i64]) -> (i32, DroidTrackingSystem) {
    let mut dts = DroidTrackingSystem::new();
    intcode::execute_op_code_with_custom_io(code, &mut dts);
    (dts.steps_to_oxygen, dts)
}

pub fn time_to_flood_the_area(dts: &DroidTrackingSystem) -> i32 {
    let mut dts = dts.clone(); // To be mutable.
    dts.current_path = vec![dts.oxygen_location];
    let mut visited: HashSet<(i32, i32)> = HashSet::from_iter(dts.current_path.iter().copied());
    let mut max_length = 0;

    'main: while !dts.current_path.is_empty() {
        for (_, pos) in dts.positions_around() {
            if dts.get_state(pos) != LocationState::Wall && !visited.contains(&pos) {
                dts.current_path.push(pos);
                visited.insert(pos);
                max_length = max_length.max(dts.current_path.len() as i32);
                continue 'main;
            }
        }

        dts.current_path.pop();
    }

    max_length - 1
}