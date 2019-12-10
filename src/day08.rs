pub fn read_from_string(raw: &str) -> Vec<u8> {
    raw.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()
}

pub fn decode_image(digits: &[u8], width: usize, height: usize) -> Vec<Vec<u8>> {
    let nb_pixel = width * height;
    let nb_layers = digits.len() / nb_pixel;
    let mut layers: Vec<Vec<u8>> = Vec::new();
    for i in 0 .. nb_layers {
        let layer: Vec<u8> = Vec::from(&digits[i * nb_pixel .. (i+1) * nb_pixel]);
        layers.push(layer);
    }
    layers
}

fn count(layer: &[u8], value: u8) -> u32 {
    layer.iter().fold(0, |sum, pix| if *pix == value { sum + 1 } else { sum })
}

pub fn layer_with_fewer_0(layers: &[Vec<u8>]) -> &Vec<u8> {
    let mut min = std::u32::MAX;
    let mut layer_min: &Vec<u8> = &layers[0];
    for layer in layers {
        let sum = count(&layer, 0u8);
        if sum < min {
            min = sum;
            layer_min = layer;
        }
    }
    layer_min
}

pub fn merge_layers(layers: &[Vec<u8>]) -> Vec<u8> {
    let size = layers[0].len();
    let mut result: Vec<u8> = Vec::new();
    result.resize(size, 0);

    for i in 0 .. size {
        for layer in layers {
            if layer[i] != 2 {
                result[i] = layer[i];
                break;
            }
        }
    }

    result
}

pub fn layer_to_printable_string(layer: &[u8], width: u32) -> String {
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

pub fn one_digits_times_two_digits(layer: &[u8]) -> u32 {
    count(layer, 1) * count(layer, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let raw = read_from_string("123456789012");
        let layers = decode_image(&raw, 3, 2);
        let layer = layer_with_fewer_0(&layers[..]);
        assert_eq!(one_digits_times_two_digits(layer), 1);
    }

    #[test]
    fn part2() {
        let raw = read_from_string("0222112222120000");
        let layers = decode_image(&raw, 2, 2);
        let layer = merge_layers(&layers[..]);
        assert_eq!(layer, vec![0, 1, 1, 0]);
    }
}