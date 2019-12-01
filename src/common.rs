use std::fs;
use std::path::Path;

pub fn read_list_of_numbers<P: AsRef<Path>>(file: P) -> Vec<i32> {
    fs::read_to_string(file).unwrap().lines().map(|line| line.parse::<i32>().unwrap()).collect()
}