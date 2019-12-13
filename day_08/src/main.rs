use std::fs;

fn main() {
    let image_as_string = fs::read_to_string("input.txt").unwrap();

    println!("Result: {}", compute_part1(&image_as_string, 25, 6));
    compute_part2(&image_as_string, 25, 6);
}

fn compute_part1(image_as_string: &str, width: i32, height: i32) -> i32 {
    let layers_count: i32 = (image_as_string.chars().count() / ((width * height) as usize)) as i32;
    assert!(image_as_string.chars().count() % ((width * height) as usize) == 0);

    println!("Layers Count: {}", layers_count);

    let digits: Vec<i32> = image_as_string.chars().map(|x| {
        match x {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!(),
        }
    }).collect();

    let mut min_zeros_count = 999_999_999;
    let mut number_of_ones = 0;
    let mut number_of_twos = 0;
    for layer_index in 0..layers_count {
        let mut zeros_count = 0;
        let mut layer_number_of_ones = 0;
        let mut layer_number_of_twos = 0;
        for x in 0..width {
            for y in 0..height {
                let index: i32 = layer_index * width * height + y * width + x;
                let digit: i32 = digits[index as usize];
                if digit == 0 {
                    zeros_count += 1;
                } else if digit == 1 {
                    layer_number_of_ones += 1;
                } else if digit == 2 {
                    layer_number_of_twos += 1;
                }
            }
        }

        if zeros_count < min_zeros_count {
            number_of_ones = layer_number_of_ones;
            number_of_twos = layer_number_of_twos;
            min_zeros_count = zeros_count;
        }
        println!("Zeros count: {}", zeros_count);
        println!("Zeros count: {}", number_of_ones * number_of_twos);
    }

    number_of_ones * number_of_twos
}

fn compute_part2(image_as_string: &str, width: i32, height: i32) -> Vec<i32> {
    let layers_count: i32 = (image_as_string.chars().count() / ((width * height) as usize)) as i32;
    assert!(image_as_string.chars().count() % ((width * height) as usize) == 0);

    println!("Layers Count: {}", layers_count);

    let source: Vec<i32> = image_as_string.chars().map(|x| {
        match x {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        }
    }).collect();

    let mut result = vec![];
    for layer_index in 0..layers_count {
        for y in 0..height {
            for x in 0..width {
                let index_for_source: i32 = layer_index * width * height + y * width + x;
                let index_for_result: i32 = y * width + x;
                let digit: i32 = source[index_for_source as usize];
                if layer_index == 0 {
                    result.push(digit);
                } else if result[index_for_result as usize] == 2 {
                    result[index_for_result as usize] = digit;
                }
            }
        }
    }

    let mut svg = String::new();
    svg.push_str(&format!("<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">", width, height));
    for y in 0..height {
        for x in 0..width {
            let index_for_result: i32 = y * width + x;
            if result[index_for_result as usize] == 0 {
                svg.push_str(&format!("<rect fill=\"black\" width=\"1\" height=\"1\" x=\"{}\" y=\"{}\" />", x, y));
            } else if result[index_for_result as usize] == 1 {
                svg.push_str(&format!("<rect fill=\"white\" width=\"1\" height=\"1\" x=\"{}\" y=\"{}\" />", x, y));
            }
        }
    }
    svg.push_str("</svg>");
    println!("{}", svg);

    result
}

#[test]
fn part1() {
    assert_eq!(1, compute_part1("123456789012", 3, 2));
    assert_eq!(4, compute_part1("000112012012", 3, 2));
}

#[test]
fn part2() {
    assert_eq!(vec![0, 1, 1, 0], compute_part2("0222112222120000", 2, 2));
}
