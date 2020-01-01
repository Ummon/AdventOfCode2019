use std::cmp::Ordering;

pub fn parse_range(raw: &str) -> (i32, i32) {
    let nums: Vec<i32> = raw.trim().split('-').map(|n| n.parse::<i32>().unwrap()).collect();
    (nums[0], nums[1])
}

type Digits = Vec<u8>;

fn get_digits(value: i32) -> Digits {
    let mut digits = Vec::<u8>::new();
    let mut value = value;
    while value > 0 {
        digits.push((value % 10) as u8);
        value /= 10;
    }
    digits
}

pub fn nb_passwords_part1(min: i32, max: i32) -> i32 {
    nb_passwords(
        min,
        max,
        &|digits: &Digits| {
            for i in 1 .. digits.len() {
                if digits[i - 1] == digits[i] { return true; }
            }
            false
        }
    )
}

pub fn nb_passwords_part2(min: i32, max: i32) -> i32 {
    nb_passwords(
        min,
        max,
        &|digits: &Digits| {
            let mut last = digits[0];
            let mut n = 1;
            for i in 1 .. digits.len() {
                if digits[i] == last {
                    n += 1;
                } else {
                    if n == 2 { return true; }
                    n = 1;
                }
                last = digits[i];
            }
            n == 2
        }
    )
}

fn nb_passwords(min: i32, max: i32, valid_password: &dyn Fn(&Digits) -> bool) -> i32 {
    let mut digits = get_digits(min);
    let digits_max = get_digits(max);
    let l = digits.len();

    fn set_range(from: usize, to: usize, value: u8, digits: &mut Digits) {
        for i in from .. to { digits[i] = value; }
    };

    for i in (1 .. l).rev() {
        if digits[i - 1] < digits[i] {
            set_range(0, i, digits[i], &mut digits);
            break;
        }
    }

    let mut n = 0;

    loop {
        if valid_password(&digits) { n += 1; }

        for i in 0 .. l {
            if i == l - 1 || digits[i + 1] <= digits[i] && digits[i] != 9 {
                set_range(0, i + 1, digits[i] + 1, &mut digits);
                break;
            }
        }

        for i in (0 .. l).rev() {
            match digits[i].cmp(&digits_max[i]) {
                Ordering::Greater => return n,
                Ordering::Less => break,
                Ordering::Equal => ()
            }
        }
    }
}