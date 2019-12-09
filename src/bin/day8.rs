use std::fs;

fn main() {
    let mut input = fs::read_to_string("resources/day8.input").unwrap();
    input.pop();

    let input = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut layers = Vec::new();

    for layer in input.chunks(25 * 6) {
        layers.push(Vec::from(layer));
    }

    let product = validate_layers(&layers);
    println!("product : {:?}", product);

    let image = generate_image(&layers);
    for row in image.chunks(25) {
        for i in row {
            print!(
                "{}",
                match i {
                    0 => '\u{2B1B}',
                    1 => '\u{2B1C}',
                    _ => panic!("Invalid color code!"),
                }
            );
        }
        println!();
    }
}

fn validate_layers(layers: &Vec<Vec<u32>>) -> u32 {
    let mut fewest_zeros = u32::max_value();
    let mut product = 0;

    for layer in layers {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        for i in layer {
            match i {
                0 => zeros = zeros + 1,
                1 => ones = ones + 1,
                2 => twos = twos + 1,
                _ => panic!("Invalid digit"),
            }
        }
        if fewest_zeros > zeros {
            fewest_zeros = zeros;
            product = ones * twos;
        }
    }
    product
}

fn generate_image(layers: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut image = vec![2; 25 * 6];
    for layer in layers {
        if merge_layers(&mut image, layer) {
            break;
        }
    }
    image
}

fn merge_layers(first: &mut Vec<u32>, second: &Vec<u32>) -> bool {
    let mut opaques = 0;
    for i in 0..first.len() {
        if first[i] == 2 {
            first[i] = second[i];
        } else {
            opaques = opaques + 1;
        }
    }
    opaques == first.len()
}
