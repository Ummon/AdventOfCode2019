// 'true' -> immediate mode, 'false' -> position mode.
fn read_op_and_modes(mut code: i32) -> (i32, [bool; 3]) {
    let mut modes: [bool; 3] = [ false, false, false ];

    if code >= 10_000 {
        modes[2] = true;
        code -= 10_000;
    }

    if code >= 1_000 {
        modes[1] = true;
        code -= 1_000;
    }

    if code >= 100 {
        modes[0] = true;
        code -= 100;
    }

    (code, modes)
}

pub fn execute_op_code(code: &mut [i32], input: &[i32]) -> Vec<i32> {
    let mut cursor = 0;
    let mut input_cursor = 0;
    let mut output = Vec::<i32>::new();

    fn read(position: usize, code: &[i32], mode: bool) -> i32 {
        if mode { code[position] } else { code[code[position] as usize] }
    };

    fn jump_if(cond: bool, cursor: usize, code: &[i32], modes: [bool; 3]) -> usize {
        let value = read(cursor + 1, &code, modes[0]);
        if cond == (value != 0) {
            read(cursor + 2, &code, modes[1]) as usize
        } else {
            cursor + 3
        }
    };

    loop {
        let (op, modes) = read_op_and_modes(code[cursor]);

        match op {
            // Sum.
            1 => {
                code[code[cursor + 3] as usize] = read(cursor + 1, &code, modes[0]) + read(cursor + 2, &code, modes[1]);
                cursor += 4;
            },

            // Multiply.
            2 => {
                code[code[cursor + 3] as usize] = read(cursor + 1, &code, modes[0]) * read(cursor + 2, &code, modes[1]);
                cursor += 4;
            },

            // Input.
            3 => {
                code[code[cursor + 1] as usize] = input[input_cursor];
                input_cursor += 1;
                cursor += 2;
            }

            // Output.
            4 => {
                output.push(read(cursor + 1, &code, modes[0]));
                cursor += 2;
            }

            // Jump-if-true.
            5 => cursor = jump_if(true, cursor, &code, modes),

            // Jump-if-false.
            6 => cursor = jump_if(false, cursor, &code, modes),

            // Less than.
            7 => {
                code[code[cursor + 3] as usize] =
                    if read(cursor + 1, &code, modes[0]) < read(cursor + 2, &code, modes[1]) { 1 } else { 0 };
                cursor += 4;
            },

            // Equals.
            8 => {
                code[code[cursor + 3] as usize] =
                    if read(cursor + 1, &code, modes[0]) == read(cursor + 2, &code, modes[1]) { 1 } else { 0 };
                cursor += 4;
            },

            99 => break,
            _ => panic!("Unkown code: {}", code[cursor])
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut c = [1002, 4, 3, 4, 33];
        let _ = execute_op_code(&mut c, &Vec::new());
        assert_eq!(c[4], 99);
    }

    #[test]
    fn part2() {
        let c = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        let mut c1 = c;
        let r1 = execute_op_code(&mut c1, &[7]);
        assert_eq!(r1[0], 999);

        let mut c2 = c;
        let r2 = execute_op_code(&mut c2, &[8]);
        assert_eq!(r2[0], 1000);

        let mut c3 = c;
        let r3 = execute_op_code(&mut c3, &[9]);
        assert_eq!(r3[0], 1001);
    }
}