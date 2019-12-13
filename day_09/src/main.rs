use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
enum Opcode {
    Add(ParamMode, ParamMode, ParamMode),
    Multiply(ParamMode, ParamMode, ParamMode),
    Input(ParamMode),
    Output(ParamMode),
    JumpIfTrue(ParamMode, ParamMode),
    JumpIfFalse(ParamMode, ParamMode),
    LessThan(ParamMode, ParamMode, ParamMode),
    Equals(ParamMode, ParamMode, ParamMode),
    AdjustBase(ParamMode),
    Halt,
}

#[derive(Debug)]
enum ProgramResult {
    Output(Program, i64),
    Halt,
}

#[derive(Debug)]
struct Program {
    program: HashMap<i64, i64>,
    pointer: i64,
    base: i64,
}

impl Program {
    fn get_opcode(&mut self) -> Opcode {
        let opcode = parse_opcode(*self.program.get(&self.pointer).unwrap_or(&0));
        self.pointer += 1;
        opcode
    }
    fn get_value(&self) -> i64 {
        *self.program.get(&self.pointer).unwrap_or(&0)
    }
    fn get_index(&self, mode: ParamMode) -> i64 {
        match mode {
            ParamMode::Position  => self.get_value(),
            ParamMode::Immediate => self.pointer,
            ParamMode::Relative  => (self.get_value() + self.base),
        }
    }
    fn get_param_value(&mut self, mode: ParamMode) -> i64 {
        let value = *self.program.get(&self.get_index(mode)).unwrap_or(&0);
        self.pointer += 1;
        value
    }
    fn set_value(&mut self, mode: ParamMode, value: i64) {
        if let ParamMode::Immediate = mode {
            assert!(false);
        }
        self.program.insert(self.get_index(mode), value);
        self.pointer += 1;
    }
    fn set_pointer(&mut self, value: i64) {
        self.pointer = value;
    }
    fn adjust_base(&mut self, value: i64) {
        self.base = self.base + value;
    }
}

fn run_program(mut program: Program, inputs: Vec<i64>) -> ProgramResult {
    let mut input_index = 0;

    loop {
        let opcode = program.get_opcode();
        // println!("{:?}", opcode);

        match opcode {
            Opcode::Add(left_mode, right_mode, result_mode) => {
                let left = program.get_param_value(left_mode);
                let right = program.get_param_value(right_mode);
                program.set_value(result_mode, left + right);
            },
            Opcode::Multiply(left_mode, right_mode, result_mode) => {
                let left = program.get_param_value(left_mode);
                let right = program.get_param_value(right_mode);
                program.set_value(result_mode, left * right);
            },
            Opcode::Input(mode) => {
                program.set_value(mode, inputs[input_index as usize]);
                input_index += 1;
            },
            Opcode::Output(mode) => {
                let output = program.get_param_value(mode);
                return ProgramResult::Output(program, output);
            },
            Opcode::JumpIfTrue(condition_mode, jump_mode) => {
                let condition = program.get_param_value(condition_mode);
                let jump = program.get_param_value(jump_mode);
                if condition > 0 {
                    program.set_pointer(jump);
                }
            },
            Opcode::JumpIfFalse(condition_mode, jump_mode) => {
                let condition = program.get_param_value(condition_mode);
                let jump = program.get_param_value(jump_mode);
                if condition == 0 {
                    program.set_pointer(jump);
                }
            },
            Opcode::LessThan(left_mode, right_mode, result_mode) => {
                if program.get_param_value(left_mode) < program.get_param_value(right_mode) {
                    program.set_value(result_mode, 1);
                } else {
                    program.set_value(result_mode, 0);
                }
            },
            Opcode::Equals(left_mode, right_mode, result_mode) => {
                if program.get_param_value(left_mode) == program.get_param_value(right_mode) {
                    program.set_value(result_mode, 1);
                } else {
                    program.set_value(result_mode, 0);
                }
            },
            Opcode::AdjustBase(mode) => {
                let new_base = program.get_param_value(mode);
                program.adjust_base(new_base);
            },
            Opcode::Halt => {
                println!("Program halted!");
                return ProgramResult::Halt;
            },
        }
    }
}

fn parse_program(program_as_string: &str) -> Program {
    let mut program = Program {
        program: HashMap::new(),
        base: 0,
        pointer: 0,
    };

    let digits: Vec<i64> = program_as_string.split(',').map(|string| {
        let i: i64 = string.parse().unwrap();
        i
    }).collect();

    for (i, digit) in digits.iter().enumerate() {
        program.program.insert(i as i64, *digit);
    }

    program
}

fn parse_param_mode(digits: i64) -> ParamMode {
    match digits {
        0 => ParamMode::Position,
        1 => ParamMode::Immediate,
        2 => ParamMode::Relative,
        _ => {
            panic!("Invalid param mode");
        },
    }
}

fn parse_opcode(digits: i64) -> Opcode {
    let opcode_as_int = digits % 100;

    match opcode_as_int {
        1 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            let result_mode = parse_param_mode((digits / 10_000) % 10);
            assert!((digits / 100_000) == 0);
            Opcode::Add(left_mode, right_mode, result_mode)
        },
        2 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            let result_mode = parse_param_mode((digits / 10_000) % 10);
            assert!((digits / 100_000) == 0);
            Opcode::Multiply(left_mode, right_mode, result_mode)
        },
        3 => {
            let mode = parse_param_mode((digits / 100) % 10);
            assert!((digits / 1000) == 0);
            Opcode::Input(mode)
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
            let result_mode = parse_param_mode((digits / 10_000) % 10);
            assert!((digits / 100_000) == 0);
            Opcode::LessThan(left_mode, right_mode, result_mode)
        },
        8 => {
            let left_mode = parse_param_mode((digits / 100) % 10);
            let right_mode = parse_param_mode((digits / 1000) % 10);
            let result_mode = parse_param_mode((digits / 10_000) % 10);
            assert!((digits / 100_000) == 0);
            Opcode::Equals(left_mode, right_mode, result_mode)
        },
        9 => {
            let mode = parse_param_mode((digits / 100) % 10);
            assert!((digits / 1000) == 0);
            Opcode::AdjustBase(mode)
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

fn main() {
    let program_as_string = fs::read_to_string("input.txt").unwrap();
    let mut program = parse_program(&program_as_string);

    let mut inputs = vec![2];
    while let ProgramResult::Output(new_program, output) = run_program(program, inputs) {
        program = new_program;
        inputs = vec![];
        println!("Result {}", output);
    }
}

fn assert_program_first_output(expected_result: i64, program_as_string: &str, inputs: Vec<i64>) {
    let output = run_program(parse_program(program_as_string), inputs);
    if let ProgramResult::Output(_, result) = output {
        assert_eq!(expected_result, result);
    }
}

#[test]
fn part1() {
    let mut program = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    for expected_output in vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99] {
        let output = run_program(program, vec![]);
        if let ProgramResult::Output(new_program, result) = output {
            program = new_program;
            println!("{}", result);
            assert_eq!(expected_output, result);
        } else {
            println!("Done!");
            break;
        }
    }

    // assert_program_first_output(1_125_899_906_842_624, "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", vec![]);
    assert_program_first_output(1_219_070_632_396_864, "1102,34915192,34915192,7,4,7,99,0", vec![]);
    assert_program_first_output(1_125_899_906_842_624, "104,1125899906842624,99", vec![]);

    assert_program_first_output(1, "3,9,8,9,10,9,4,9,99,-1,8", vec![8]);
    assert_program_first_output(0, "3,9,8,9,10,9,4,9,99,-1,8", vec![7]);
    assert_program_first_output(0, "3,9,8,9,10,9,4,9,99,-1,8", vec![9]);

    assert_program_first_output(1, "3,3,1108,-1,8,3,4,3,99", vec![8]);
    assert_program_first_output(0, "3,3,1108,-1,8,3,4,3,99", vec![7]);
    assert_program_first_output(0, "3,3,1108,-1,8,3,4,3,99", vec![9]);

    assert_program_first_output(0, "3,9,7,9,10,9,4,9,99,-1,8", vec![8]);
    assert_program_first_output(1, "3,9,7,9,10,9,4,9,99,-1,8", vec![7]);
    assert_program_first_output(0, "3,9,7,9,10,9,4,9,99,-1,8", vec![9]);

    assert_program_first_output(0, "3,3,1107,-1,8,3,4,3,99", vec![8]);
    assert_program_first_output(1, "3,3,1107,-1,8,3,4,3,99", vec![7]);
    assert_program_first_output(0, "3,3,1107,-1,8,3,4,3,99", vec![9]);

    assert_program_first_output(0, "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0]);
    assert_program_first_output(1, "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![42]);
    assert_program_first_output(1, "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![1]);

    assert_program_first_output(0, "3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0]);
    assert_program_first_output(1, "3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![42]);
    assert_program_first_output(1, "3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![1]);
    
    assert_program_first_output(1000, "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", vec![8]);
    assert_program_first_output(999, "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", vec![7]);
    assert_program_first_output(1001, "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", vec![9]);
}