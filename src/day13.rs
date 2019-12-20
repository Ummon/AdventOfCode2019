use super::intcode;
use std::cmp::Ordering;
use std::convert::TryFrom;
use itertools::Itertools;
use num_enum::TryFromPrimitive;

// Not neccesary, to try to parse enum in Rust.
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

pub fn count_nb_block(code: &[i64]) -> i32 {
    let output = intcode::execute_op_code(code, &[]);

    output.iter().chunks(3).into_iter().map(
        |tile_with_pos| {
            let tile_with_pos: Vec<&i64> = tile_with_pos.collect();
            let tile = Tile::try_from(*tile_with_pos[2] as u8).unwrap();
            if tile == Tile::Block { 1 } else { 0 }
        }
    ).sum()
}

struct State {
    score: i64,
    joystick: i64, // -1: move to left, 0: do not move, 1: move to right.
    paddle_position_x: i64,
    ball_position_x: i64,
    buffer: Vec<i64>, // Used to buffer the input.
}

impl intcode::IO for State {
    fn read(&mut self) -> i64 {
        self.joystick
    }

    fn write(&mut self, value: i64) {
        self.buffer.push(value);
        if self.buffer.len() == 3 {
            if self.buffer[0] == -1 && self.buffer[1] == 0 {
                self.score = self.buffer[2];
            } else {
                let tile = Tile::try_from(self.buffer[2] as u8).unwrap();
                if tile == Tile::Ball {
                    self.ball_position_x = self.buffer[0];
                } else if tile == Tile::Paddle {
                    self.paddle_position_x = self.buffer[0];
                }
                self.joystick =
                    match self.paddle_position_x.cmp(&self.ball_position_x) {
                        Ordering::Greater => -1,
                        Ordering::Less => 1,
                        Ordering::Equal => 0
                    };
            }
            self.buffer.clear();
        }
    }
}

pub fn final_score(code: &[i64]) -> i64 {
    let mut state = State { score: 0, joystick: 0, paddle_position_x: 0, ball_position_x: 0, buffer: Vec::new() };
    intcode::execute_op_code_with_custom_io(&code, &mut state);
    state.score
}
