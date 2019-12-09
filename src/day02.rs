pub fn execute_op_code_with_state_fixed(code: &mut [i32]) -> i32 {
    code[1] = 12;
    code[2] = 2;
    execute_op_code(code)
}

fn execute_op_code(code: &mut [i32]) -> i32 {
    let mut cursor = 0;
    loop {
        match code[cursor] {
            1 => code[code[cursor + 3] as usize] = code[code[cursor + 1] as usize] + code[code[cursor + 2] as usize],
            2 => code[code[cursor + 3] as usize] = code[code[cursor + 1] as usize] * code[code[cursor + 2] as usize],
            99 => return code[0],
            _ => panic!("Unkown code: {}", code[cursor])
        }
        cursor += 4;
    }
}

pub fn find_noun_and_verb(code: &[i32]) -> i32 {
    loop {
        for verb in 0..=99 {
            for noun in 0..=99 {
                let mut code_copy = Vec::from(code);
                code_copy[1] = noun;
                code_copy[2] = verb;
                if execute_op_code(&mut code_copy) == 19_690_720 {
                    return 100 * noun + verb
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cases() {
        let mut c1 = [1, 0, 0, 0, 99];
        execute_op_code(&mut c1);
        assert_eq!(c1[0], 2);

        let mut c2 = [2, 3, 0, 3, 99];
        execute_op_code(&mut c2);
        assert_eq!(c2[3], 6);

        let mut c3 = [2, 4, 4, 5, 99, 0];
        execute_op_code(&mut c3);
        assert_eq!(c3[5], 9801);

        let mut c4 = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute_op_code(&mut c4);
        assert_eq!(c4[0], 30);
    }
}