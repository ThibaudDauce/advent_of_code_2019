use std::fs;

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
}

#[derive(Debug)]
enum Opcode {
    Add(ParamMode, ParamMode),
    Multiply(ParamMode, ParamMode),
    Input,
    Output(ParamMode),
    JumpIfTrue(ParamMode, ParamMode),
    JumpIfFalse(ParamMode, ParamMode),
    LessThan(ParamMode, ParamMode),
    Equals(ParamMode, ParamMode),
    Halt,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let program: Vec<i32> = input.split(',').map(|string| {
        let i: i32 = string.parse().unwrap();
        i
    }).collect();

    println!("{}", compute_part1(program.clone(), 1));
    println!("{}", compute_part1(program.clone(), 5));
}

fn get_index(program: &[i32], mode: ParamMode, i: i32) -> usize {
    (match mode {
        ParamMode::Position  => program[i as usize],
        ParamMode::Immediate => i,
    }) as usize
}

fn compute_part1(mut program: Vec<i32>, input: i32) -> i32 {
    let mut i: i32 = 0;
    let mut output: i32 = 99999;
    loop {
        let opcode = parse_opcode(program[i as usize]);

        // println!("{}: {:?}", i, opcode);

        match opcode {
            Opcode::Add(left_mode, right_mode) => {
                let left_index = get_index(&program, left_mode, i + 1);
                let right_index = get_index(&program, right_mode, i + 2);
                let result_index = get_index(&program, ParamMode::Position, i + 3);

                program[result_index] = program[left_index] + program[right_index];
                i += 4;
            },
            Opcode::Multiply(left_mode, right_mode) => {
                let left_index = get_index(&program, left_mode, i + 1);
                let right_index = get_index(&program, right_mode, i + 2);
                let result_index = get_index(&program, ParamMode::Position, i + 3);

                program[result_index] = program[left_index] * program[right_index];
                i += 4;
            },
            Opcode::Input => {
                let index = program[(i + 1) as usize];
                program[index as usize] = input;
                println!("Input {} set at {}", input, index);
                i += 2;
            },
            Opcode::Output(mode) => {
                let index = get_index(&program, mode, i + 1);
                output = program[index];
                println!("Output is {}", output);
                i += 2;
            },
            Opcode::JumpIfTrue(condition_mode, jump_mode) => {
                let condition_index = get_index(&program, condition_mode, i + 1);
                if program[condition_index] > 0 {
                    let jump_index = get_index(&program, jump_mode, i + 2);
                    i = program[jump_index];
                } else {
                    i += 3;
                }
            },
            Opcode::JumpIfFalse(condition_mode, jump_mode) => {
                let condition_index = get_index(&program, condition_mode, i + 1);
                if program[condition_index] == 0 {
                    let jump_index = get_index(&program, jump_mode, i + 2);
                    i = program[jump_index];
                } else {
                    i += 3;
                }
            },
            Opcode::LessThan(left_mode, right_mode) => {
                let left_index = get_index(&program, left_mode, i + 1);
                let right_index = get_index(&program, right_mode, i + 2);
                let result_index = get_index(&program, ParamMode::Position, i + 3);
                if program[left_index] < program[right_index] {
                    program[result_index] = 1;
                } else {
                    program[result_index] = 0;
                }
                i += 4;
            },
            Opcode::Equals(left_mode, right_mode) => {
                let left_index = get_index(&program, left_mode, i + 1);
                let right_index = get_index(&program, right_mode, i + 2);
                let result_index = get_index(&program, ParamMode::Position, i + 3);
                if program[left_index] == program[right_index] {
                    program[result_index] = 1;
                } else {
                    program[result_index] = 0;
                }
                i += 4;
            },
            Opcode::Halt => {
                println!("Program halted!");
                return output
            },
        }
    }
}

fn parse_param_mode(digits: i32) -> ParamMode {
    match digits {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        _ => {
            panic!("Invalid param mode");
        },
    }
}

fn parse_opcode(digits: i32) -> Opcode {
    let opcode_as_int = digits % 100;

    match opcode_as_int {
        1 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            assert!((digits / 10_000) == 0);
            Opcode::Add(left_mode, right_mode)
        },
        2 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            assert!((digits / 10_000) == 0);
            Opcode::Multiply(left_mode, right_mode)
        },
        3 => {
            assert!((digits / 100) == 0);
            Opcode::Input
        },
        4 => {
            let mode = parse_param_mode((digits / 100) % 10);
            assert!((digits / 1000) == 0);
            Opcode::Output(mode)
        },
        5 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            assert!((digits / 10_000) == 0);
            Opcode::JumpIfTrue(left_mode, right_mode)
        },
        6 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            assert!((digits / 10_000) == 0);
            Opcode::JumpIfFalse(left_mode, right_mode)
        },
        7 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            assert!((digits / 10_000) == 0);
            Opcode::LessThan(left_mode, right_mode)
        },
        8 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            assert!((digits / 10_000) == 0);
            Opcode::Equals(left_mode, right_mode)
        },
        99 => {
            assert!((digits / 100) == 0);
            Opcode::Halt
        },
        _ => {
            panic!("Invalid opcode {}", digits);
        },
    }
}

#[test]
fn part2() {
    assert_eq!(1, compute_part1(vec![3,9,8,9,10,9,4,9,99,-1,8], 8));
    assert_eq!(0, compute_part1(vec![3,9,8,9,10,9,4,9,99,-1,8], 7));
    assert_eq!(0, compute_part1(vec![3,9,8,9,10,9,4,9,99,-1,8], 9));

    assert_eq!(1, compute_part1(vec![3,3,1108,-1,8,3,4,3,99], 8));
    assert_eq!(0, compute_part1(vec![3,3,1108,-1,8,3,4,3,99], 7));
    assert_eq!(0, compute_part1(vec![3,3,1108,-1,8,3,4,3,99], 9));

    assert_eq!(0, compute_part1(vec![3,9,7,9,10,9,4,9,99,-1,8], 8));
    assert_eq!(1, compute_part1(vec![3,9,7,9,10,9,4,9,99,-1,8], 7));
    assert_eq!(0, compute_part1(vec![3,9,7,9,10,9,4,9,99,-1,8], 9));

    assert_eq!(0, compute_part1(vec![3,3,1107,-1,8,3,4,3,99], 8));
    assert_eq!(1, compute_part1(vec![3,3,1107,-1,8,3,4,3,99], 7));
    assert_eq!(0, compute_part1(vec![3,3,1107,-1,8,3,4,3,99], 9));

    assert_eq!(0, compute_part1(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0));
    assert_eq!(1, compute_part1(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 42));
    assert_eq!(1, compute_part1(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 1));

    assert_eq!(0, compute_part1(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0));
    assert_eq!(1, compute_part1(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 42));
    assert_eq!(1, compute_part1(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 1));
    
    assert_eq!(1000, compute_part1(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8));
    assert_eq!(999, compute_part1(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 7));
    assert_eq!(1001, compute_part1(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 9));
}