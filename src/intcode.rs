#[derive(Copy, Clone, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative
}

pub trait IO {
    fn read(&mut self) -> i64;
    fn write(&mut self, value: i64);
    fn halt(&self) -> bool { false }
    fn finished(&mut self) { }
}

struct Buffer {
    output: Vec<i64>,
    input: Vec<i64>
}

impl Buffer {
    fn from(input: &[i64]) -> Self {
        Buffer {
            output: Vec::new(),
            input: Vec::from(input)
        }
    }
}

impl IO for Buffer {
    fn read(&mut self) -> i64 {
        self.input.remove(0)
    }

    fn write(&mut self, value: i64) {
        self.output.push(value)
    }
}

// 'true' -> immediate mode, 'false' -> position mode.
fn read_op_and_modes(mut code: i64) -> (i64, [Mode; 3]) {
    let mut modes: [Mode; 3] = [ Mode::Position, Mode::Position, Mode::Position ];

    for i in (2..=4).rev() {
        let power = 10i64.pow(i);
        if code >= 2 * power {
            modes[i as usize - 2] = Mode::Relative;
            code -= 2 * power;
        } else if code >= power {
            modes[i as usize - 2] = Mode::Immediate;
            code -= power;
        }
    }

    (code, modes)
}

pub fn execute_op_code(code: &[i64], input: &[i64]) -> Vec<i64> {
    let mut buffer = Buffer::from(input);
    execute_op_code_with_custom_io(code, &mut buffer);
    buffer.output
}

pub fn execute_op_code_with_custom_io(code: &[i64], io: &mut dyn IO) {
    let mut code = Vec::from(code);
    let mut cursor = 0;
    let mut relative_base = 0;

    fn read(position: usize, code: &[i64], mode: Mode, relative_base: i64) -> i64 {
        if position >= code.len() { return 0 }

        match mode {
            Mode::Position => if code[position] as usize >= code.len() { 0 } else { code[code[position] as usize] },
            Mode::Immediate => code[position],
            Mode::Relative => if (code[position] + relative_base) as usize >= code.len() { 0 } else { code[(code[position] + relative_base) as usize] }
        }
    };

    fn write(position: usize, value: i64, code: &mut Vec<i64>, mode: Mode, relative_base: i64) {
        let ref_position =
            match mode {
                Mode::Position => code[position] as usize,
                Mode::Immediate => panic!("Can't write un immediate mode"),
                Mode::Relative => (code[position] + relative_base) as usize
            };

        if ref_position >= code.len() {
            code.resize(ref_position + 1, 0);
        }

        code[ref_position] = value;
    }

    fn jump_if(cond: bool, cursor: usize, code: &[i64], modes: [Mode; 3], relative_base: i64) -> usize {
        let value = read(cursor + 1, &code, modes[0], relative_base);
        if cond == (value != 0) {
            read(cursor + 2, &code, modes[1], relative_base) as usize
        } else {
            cursor + 3
        }
    };

    loop {
        if io.halt() { break; }

        let (op, modes) = read_op_and_modes(code[cursor]);

        match op {
            // Sum.
            1 => {
                write(cursor + 3, read(cursor + 1, &code, modes[0], relative_base) + read(cursor + 2, &code, modes[1], relative_base), &mut code, modes[2], relative_base);
                cursor += 4;
            },

            // Multiply.
            2 => {
                write(cursor + 3, read(cursor + 1, &code, modes[0], relative_base) * read(cursor + 2, &code, modes[1], relative_base), &mut code, modes[2], relative_base);
                cursor += 4;
            },

            // Input.
            3 => {
                write(cursor + 1, io.read(), &mut code, modes[0], relative_base);
                cursor += 2;
            }

            // Output.
            4 => {
                io.write(read(cursor + 1, &code, modes[0], relative_base));
                cursor += 2;
            }

            // Jump-if-true.
            5 => cursor = jump_if(true, cursor, &code, modes, relative_base),

            // Jump-if-false.
            6 => cursor = jump_if(false, cursor, &code, modes, relative_base),

            // Less than.
            7 => {
                write(cursor + 3, if read(cursor + 1, &code, modes[0], relative_base) < read(cursor + 2, &code, modes[1], relative_base) { 1 } else { 0 }, &mut code, modes[2], relative_base);
                cursor += 4;
            },

            // Equals.
            8 => {
                write(cursor + 3, if read(cursor + 1, &code, modes[0], relative_base) == read(cursor + 2, &code, modes[1], relative_base) { 1 } else { 0 }, &mut code, modes[2], relative_base);
                cursor += 4;
            },

            // Change relative base.
            9 => {
                relative_base += read(cursor + 1, &code, modes[0], relative_base);
                cursor += 2;
            }

            99 => break,

            _ => panic!("Unknown code: {}", code[cursor])
        }
    }

    io.finished();
}

#[cfg(test)]
mod tests_day05 {
    use super::*;

    #[test]
    fn part2() {
        let c = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let r1 = execute_op_code(&c, &[7]);
        assert_eq!(r1[0], 999);

        let r2 = execute_op_code(&c, &[8]);
        assert_eq!(r2[0], 1000);

        let r3 = execute_op_code(&c, &[9]);
        assert_eq!(r3[0], 1001);
    }
}

#[cfg(test)]
mod tests_day09 {
    use super::*;

    #[test]
    fn copy_of_itself() {
        let c = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let r = execute_op_code(&c, &Vec::new());
        assert_eq!(r, c);
    }

    #[test]
    fn output_big_number() {
        let c = [1102,34_915_192,34_915_192,7,4,7,99,0];
        let r = execute_op_code(&c, &Vec::new());
        assert_eq!(r[0], 1_219_070_632_396_864);
    }

    #[test]
    fn input_big_number() {
        let c = [104,1_125_899_906_842_624,99];
        let r = execute_op_code(&c, &Vec::new());
        assert_eq!(r[0], 1_125_899_906_842_624);
    }
}