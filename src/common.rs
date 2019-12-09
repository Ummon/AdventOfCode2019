use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn read_list_of_numbers<P, T>(file: P, sep: &str) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
    T::Err: std::fmt::Debug

{
    fs::read_to_string(file).unwrap().split(sep).map(|line| line.parse::<T>().unwrap()).collect()
}