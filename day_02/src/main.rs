use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut program: Vec<usize> = input.split(',').map(|string| {
        let i: usize = string.parse().unwrap();
        i
    }).collect();

    program[1] = 12;
    program[2] = 2;

    let result_program = compute_part1(program.clone());

    println!("{}", result_program[0]);

    for noun in 1..100 {
        for verb in 1..100 {
            println!("noun = {}; verb = {}", noun, verb);
            program[1] = noun;
            program[2] = verb;
            let result_program = compute_part1(program.clone());

            println!("{}", result_program[0]);
            if result_program[0] == 19_690_720 {
                println!("Result = {}", 100 * noun + verb);
                return
            }
        }
    }
}

fn compute_part1(mut program: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    while program[i] != 99 {
        let opcode = program[i];
        let left_index = program[i + 1];
        let right_index = program[i + 2];
        let result_index = program[i + 3];

        if opcode == 1 {
            program[result_index] = program[left_index] + program[right_index];
        } else if opcode == 2 {
            program[result_index] = program[left_index] * program[right_index];
        } else {
            panic!("Invalid opcode");
        }

        i += 4;
    }
    program
}

#[test]
fn part1() {
    assert_eq!(vec![2,0,0,0,99], compute_part1(vec![1,0,0,0,99]));
    assert_eq!(vec![2,3,0,6,99], compute_part1(vec![2,3,0,3,99]));
    assert_eq!(vec![2,4,4,5,99,9801], compute_part1(vec![2,4,4,5,99,0]));
    assert_eq!(vec![30,1,1,4,2,5,6,0,99], compute_part1(vec![1,1,1,4,99,5,6,0,99]));
}