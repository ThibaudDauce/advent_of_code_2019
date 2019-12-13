fn main() {
    {
        let mut sum = 0;
        for password in 240_920..789_857 {
            if compute_part1(break_password(password)) {
                sum += 1;
            }
        }
    
        println!("{}", sum);
    }

    {
        let mut sum = 0;
        for password in 240_920..789_857 {
            if compute_part2(break_password(password)) {
                sum += 1;
            }
        }
    
        println!("{}", sum);
    }
}

fn break_password(digits: i32) -> [i32; 6] {
    let digit_1 = digits / 100_000;
    let rem_1   = digits.rem_euclid(100_000);

    let digit_2 = rem_1 / 10_000;
    let rem_2   = rem_1.rem_euclid(10_000);

    let digit_3 = rem_2 / 1_000;
    let rem_3   = rem_2.rem_euclid(1_000);

    let digit_4 = rem_3 / 100;
    let rem_4   = rem_3.rem_euclid(100);

    let digit_5 = rem_4 / 10;
    let rem_5   = rem_4.rem_euclid(10);

    let digit_6 = rem_5;

    [digit_1, digit_2, digit_3, digit_4, digit_5, digit_6]
}

fn compute_part1(password: [i32; 6]) -> bool {
    let mut previous_digit = None;
    let mut double_digits = false;
    for digit in password.iter() {
        if let Some(previous_digit) = previous_digit {
            if previous_digit > digit {
                return false;
            }

            if previous_digit == digit {
                double_digits = true;
            }
        }

        previous_digit = Some(digit);
    }
    
    double_digits
}

fn compute_part2(password: [i32; 6]) -> bool {
    let mut previous_digit = None;
    let mut double_digits = false;
    let mut repetition_count = 1;
    for digit in password.iter() {
        if let Some(previous_digit) = previous_digit {
            if previous_digit > digit {
                return false;
            }

            if previous_digit == digit {
                repetition_count += 1;
            } else {
                if repetition_count == 2 {
                    double_digits = true;
                }
                repetition_count = 1;
            }
        }

        previous_digit = Some(digit);
    }

    if repetition_count == 2 {
        double_digits = true;
    }
    
    double_digits
}

#[test]
fn part1() {
    assert_eq!(true, compute_part1([1, 1, 1, 1, 1, 1]));
    assert_eq!(false, compute_part1([2, 2, 3, 4, 5, 0]));
    assert_eq!(false, compute_part1([1, 2, 3, 7, 8, 9]));
}

#[test]
fn part2() {
    assert_eq!(true, compute_part2([1, 1, 2, 2, 3, 3]));
    assert_eq!(false, compute_part2([1, 2, 3, 4, 4, 4]));
    assert_eq!(true, compute_part2([1, 1, 1, 1, 2, 2]));
}
