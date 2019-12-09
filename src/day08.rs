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

pub fn layer_with_fewer_0(layers: &[Vec<u8>]) -> &Vec<u8> {
    let mut min = std::u32::MAX;
    let mut layer_min = 0;
    for i in 0 .. layers.len() {
        let sum = layers[i].iter().fold(0, |sum, pix| if *pix == 0u8 { sum + 1 } else { sum });
        if sum < min {
            min = sum;
            layer_min = i;
        }
    }
    &layers[layer_min]
}

pub fn one_digits_times_two_digits(layer: &[u8]) -> i32 {
    let (nb_1, nb_2) =
        layer
            .iter()
            .fold(
                (0, 0),
                |(nb_1, nb_2), pix|
                    match *pix { 1u8 => (nb_1 + 1, nb_2), 2u8 => (nb_1, nb_2 + 1), _ => (nb_1, nb_2)}
            );
    nb_1 * nb_2
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
}