use std::collections::HashMap;

fn parse_quantity_and_name(input: &str) -> (i32, String) {
    let quantity_and_name: Vec<&str> = input.split(" ").collect();
    (
        quantity_and_name[0].parse::<i32>().unwrap(),
        String::from(quantity_and_name[1].trim())
    )
}

fn parse(input: &str) -> HashMap<String, (i32, Vec<(i32, String)>)> {
    let result = HashMap::<String, (i32, Vec<(i32, String)>)>::new();
    for line in input.lines() {
        let reaction: Vec<&str> = line.split("=>").collect();
        let input_chemicals: Vec<(i32, String)> =
            reaction[0].split(",").map(parse_quantity_and_name).collect();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {

    }
}