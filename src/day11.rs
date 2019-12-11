use std::collections::HashMap;
use super::intcode;

enum NextCommand {
    ColorToPaint,
    Turn
}

struct Robot {
    next_command: NextCommand,
    current_pos: (i32, i32),
    current_dir: i32, // 0: up, 1: right, 2: down, 3: left.
    panels: HashMap<(i32, i32), i64>
}

impl Robot {
    fn new() -> Self {
        Robot {
            next_command: NextCommand::ColorToPaint,
            current_pos: (0, 0),
            current_dir: 0,
            panels: HashMap::new()
        }
    }
}

impl intcode::IO for Robot {
    fn read(&mut self) -> i64 {
        *self.panels.get(&self.current_pos).unwrap_or(&0)
    }

    fn write(&mut self, value: i64) {
        self.next_command =
            match self.next_command {
                NextCommand::ColorToPaint => { self.panels.insert(self.current_pos, value); NextCommand::Turn },
                NextCommand::Turn => {
                    self.current_dir = (self.current_dir + if value == 0 { 3 } else { 1 }) % 4;
                    self.current_pos =
                        match self.current_dir {
                            0 => (self.current_pos.0, self.current_pos.1 + 1),
                            1 => (self.current_pos.0 + 1, self.current_pos.1),
                            2 => (self.current_pos.0, self.current_pos.1 - 1),
                            3 | _ => (self.current_pos.0 - 1, self.current_pos.1)
                        };
                    NextCommand::ColorToPaint
                }
            }
    }
}

pub fn run_robot(code: &[i64], initial_value: i64) -> HashMap<(i32, i32), i64> {
    let mut robot = Robot::new();
    if initial_value != 0 {
        robot.panels.insert((0, 0), initial_value);
    }

    intcode::execute_op_code_with_custom_io(code, &mut robot);
    robot.panels
}

pub fn panels_to_layer(panels: &HashMap<(i32, i32), i64>) -> (Vec<u8>, usize) {
    let coordinates: Vec<&(i32, i32)> = panels.keys().collect();
    let min_x = coordinates.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = coordinates.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = coordinates.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = coordinates.iter().max_by_key(|(_, y)| y).unwrap().1;

    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;

    let mut layer = Vec::new();
    layer.resize(width * height, 0);

    for x in min_x ..= max_x {
        for y in min_y ..= max_y {
            let pos = (x - min_x) + ((height as i32 - y + min_y - 1) * width as i32); // Y axis is down.
            layer[pos as usize] = *panels.get(&(x, y)).unwrap_or(&0) as u8;
        }
    }

    (layer, width)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::intcode::IO;

    #[test]
    fn part1() {
        let mut robot = Robot::new();
        robot.write(1); // Paint white.
        robot.write(0); // Turn left.

        robot.write(0); // Paint black.
        robot.write(0); // Turn left.

        robot.write(1); // Paint white.
        robot.write(0); // Turn left.

        robot.write(1); // Paint white.
        robot.write(0); // Turn left.

        robot.write(0); // Paint black.
        robot.write(1); // Turn right.

        robot.write(1); // Paint white.
        robot.write(0); // Turn left.

        robot.write(1); // Paint white.
        robot.write(0); // Turn left.

        assert_eq!(robot.panels.len(), 6);
    }

}