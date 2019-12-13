use std::fs;

fn main() {
    let all_permutations = permutations(&[0, 1, 2, 3, 4], vec![], vec![]);
    let content = fs::read_to_string("input.txt").unwrap();
    println!("{}", get_max_output(&content, &all_permutations));
    
    let all_permutations = permutations(&[5, 6, 7, 8, 9], vec![], vec![]);
    let content = fs::read_to_string("input.txt").unwrap();
    println!("{}", get_max_output_part2(&content, &all_permutations));
}

fn get_max_output(content: &str, permutations: &Vec<Vec<i32>>) -> i32 {
    let program: Vec<i32> = content.split(',').map(|string| {
        let i: i32 = string.parse().unwrap();
        i
    }).collect();

    let mut max_output = 0;
    for permutation in permutations {
        let mut output = 0;
        for phase in permutation {
            if let HaltMode::Output(program_output, _, _) = run_program(program.clone(), &[*phase, output], 0) {
                output = program_output;
            }
        }

        if output > max_output {
            max_output = output;
        }
    }

    max_output
}

fn get_max_output_part2(content: &str, permutations: &Vec<Vec<i32>>) -> i32 {
    let program: Vec<i32> = content.split(',').map(|string| {
        let i: i32 = string.parse().unwrap();
        i
    }).collect();

    let mut max_output = 0;
    for permutation in permutations {
        println!();
        println!("#####################$$");
        println!("Try permutations {:?}", permutation);
        println!("#####################$$");
        println!();
        println!();

        let mut output = 0;
        let mut program_index = 0;
        let mut programs: Vec<Option<(Vec<i32>, i32)>> = vec![None, None, None, None, None];
        loop {
            let (program_to_run, start_pointer, inputs): (Vec<i32>, i32, Vec<i32>) = if let Some((ampli_program, pointer)) = &programs[program_index] {
                (ampli_program.clone(), *pointer, vec![output])
            } else {
                (program.clone(), 0, vec![permutation[program_index], output])
            };
            if let HaltMode::Output(program_output, modified_program, modified_pointer) = run_program(program_to_run, &inputs, start_pointer) {
                output = program_output;
                programs[program_index] = Some((modified_program, modified_pointer));

                program_index += 1;
                if program_index >= 5 {
                    program_index = 0;
                }
            } else {
                break;
            }
        }

        if output > max_output {
            max_output = output;
        }
    }

    max_output
}

fn permutations(digits: &[i32], current: Vec<i32>, mut all_permutations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if digits.len() == current.len() {
        all_permutations.push(current);
        return all_permutations;
    }

    for digit in digits {
        if !current.contains(&digit) {
            let mut new_current = current.clone();
            new_current.push(*digit);
            all_permutations = permutations(&digits, new_current, all_permutations);
        }
    }

    all_permutations
}

#[test]
fn part1() {
    let permutations = permutations(&[0, 1, 2, 3, 4], vec![], vec![]);
    assert_eq!(43210, get_max_output("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", &permutations));
    assert_eq!(54321, get_max_output("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", &permutations));
    assert_eq!(65210, get_max_output("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", &permutations));
}

#[test]
fn part2() {
    let permutations = permutations(&[5, 6, 7, 8, 9], vec![], vec![]);
    assert_eq!(139_629_729, get_max_output_part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", &vec![vec![9,8,7,6,5]]));
    assert_eq!(18216, get_max_output_part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", &permutations));
}

























#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
}

#[derive(Debug)]
enum HaltMode {
    Output(i32, Vec<i32>, i32),
    Halt,
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

fn get_index(program: &[i32], mode: ParamMode, i: i32) -> usize {
    (match mode {
        ParamMode::Position  => program[i as usize],
        ParamMode::Immediate => i,
    }) as usize
}

fn run_program(mut program: Vec<i32>, inputs: &[i32], mut i: i32) -> HaltMode {
    let mut input_index = 0;
    let output: i32;

    // println!("Start program with inputs {:?}", inputs);

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
                program[index as usize] = inputs[input_index];
                // println!("Input {} set at {}", inputs[input_index], index);
                input_index += 1;
                i += 2;
            },
            Opcode::Output(mode) => {
                let index = get_index(&program, mode, i + 1);
                output = program[index];
                // println!("Output is {}", output);
                i += 2;
                return HaltMode::Output(output, program, i);
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
                return HaltMode::Halt;
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