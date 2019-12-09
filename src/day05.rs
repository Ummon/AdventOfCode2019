fn read_code(code: i32) -> (i32, Vec<bool>) {

}

fn execute_op_code(code: &mut [i32], input: &[i32]) -> Vec<i32> {
    let mut cursor = 0;
    let mut output = Vec::<i32>::new();
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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
    }

    #[test]
    fn part2() {
    }
}