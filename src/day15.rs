use super::intcode;

struct RepairDroid {
}

impl intcode::IO for RepairDroid {
    // May block.
    fn read(&mut self) -> i64 {
        1
    }

    // Send to the output channel.
    fn write(&mut self, value: i64) {
    }
}

pub fn shorted_path_to_oxygen(code: &[i64]) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_1() {

    }
}