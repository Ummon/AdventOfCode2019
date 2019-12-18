use std::iter::FromIterator;

pub fn parse(input: &str) -> Vec<i32> {
    input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
}

pub fn fft(signal: &[i32], pattern: &[i32], nb_phases: i32, offset: usize, length: usize, nb_signal_repeated: usize) -> Vec<i32> {

    let l = signal.len();
    let pattern_l = pattern.len();

    let mut output = Vec::from_iter(signal.iter().cycle().take(l * nb_signal_repeated).copied());

    for _ in 0 .. nb_phases {
        let cloned_output = output.clone();
        for i in 0 .. output.len() {
            output[i] =
                cloned_output.iter().enumerate().fold(
                    0,
                    |sum, (j, value)| {
                        sum + value * pattern[(j + 1) / (i + 1) % pattern_l]
                    }
                ).abs() % 10;
        }
    }

    Vec::from(&output[offset .. offset + length])
}

pub fn digits_as_string(signal: &[i32]) -> String {
    signal.iter().fold(String::new(), |result, digit| result + &digit.to_string())
}

// Part 2 is from 'https://github.com/mkeeter/advent-of-code/blob/master/2019/16/src/main.rs'.

fn cumsum(input: &[i32]) -> Vec<i32> {
    let mut output = vec![0; input.len() + 1];
    for (i, v) in input.iter().enumerate() {
        output[i + 1] = output[i] + v;
    }
    output
}

fn dft(scale: usize, csum: &[i32]) -> i32 {
    use std::cmp::min;

    assert!(scale > 0);
    let mut i = scale;
    let mut sign = true;
    let mut out = 0;
    while i < csum.len() {
        let d = csum[min(csum.len() - 1, i + scale - 1)] - csum[i - 1];
        if sign {
            out += d;
        } else {
            out -= d;
        }
        sign = !sign;
        i += scale * 2;
    }
    out.abs() % 10
}

#[allow(dead_code)]
pub fn part2(input: &[i32]) -> Vec<i32> {
    let size = input.len();
    let mut input = Vec::from_iter(input.iter().cycle().take(size * 10_000).copied());

    let offset = input[..7].iter().fold(0, |acc, i| acc * 10 + i) as usize;

    for _ in 0..100 {
        let csum = cumsum(&input);
        input = (0..input.len())
            .map(|i| dft(i + 1, &csum))
            .collect::<Vec<i32>>();
    }

    Vec::from(&input[offset..(offset + 8)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_1() {
        let signal = parse("80871224585914546619083218645595");
        let output = fft(&signal, &[0, 1, 0, -1], 100, 0, 8, 1);
        assert_eq!(digits_as_string(&output), "24176176");
    }

    #[test]
    fn part1_sample_2() {
        let signal = parse("19617804207202209144916044189917");
        let output = fft(&signal, &[0, 1, 0, -1], 100, 0, 8, 1);
        assert_eq!(digits_as_string(&output), "73745418");
    }

    #[test]
    fn part1_sample_3() {
        let signal = parse("69317163492948606335995924319873");
        let output = fft(&signal, &[0, 1, 0, -1], 100, 0, 8, 1);
        assert_eq!(digits_as_string(&output), "52432133");
    }
}