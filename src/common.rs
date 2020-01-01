use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn read_list_of_numbers<P, T>(file: P, sep: &str) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
    T::Err: std::fmt::Debug

{
    fs::read_to_string(file).unwrap().split(sep).map(|line| line.trim().parse::<T>().unwrap()).collect()
}

pub fn layer_to_printable_string(layer: &[u8], width: usize) -> String {
    let mut result = String::new();
    let mut i = 0;

    loop {
        for _ in 0 .. width {
            if layer[i] == 0 {
                result += " ";
            } else {
                result += "█";
            }
            i += 1;
            if i >= layer.len() { return result }
        }
        result += "\n";
    }
}