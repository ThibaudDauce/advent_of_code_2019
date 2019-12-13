use std::collections::HashMap;
use std::fs;

fn main() {
    let program_as_string = fs::read_to_string("input.txt").unwrap();
    let mut program = parse_program(&program_as_string);
    let mut mode = Mode::Color;
    let mut position = (0, 0);
    let mut colors: HashMap<(i64, i64), i64> = HashMap::new();
    let mut direction = Direction::Up;

    let mut count = 0;

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    let mut inputs = vec![1];
    while let ProgramResult::Output(new_program, output) = run_program(program, inputs) {
        match mode {
            Mode::Color => {
                if output != 0 && output != 1 {
                    panic!();
                }
                if colors.get(&position).is_none() {
                    count += 1;
                }
                colors.insert(position, output);
                mode = Mode::Rotation;
                inputs = vec![];
            },
            Mode::Rotation => {
                direction = match output {
                    0 => {
                        match direction {
                            Direction::Up => Direction::Left,
                            Direction::Right => Direction::Up,
                            Direction::Bottom => Direction::Right,
                            Direction::Left => Direction::Bottom,
                        }
                    },
                    1 => {
                        match direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Bottom,
                            Direction::Bottom => Direction::Left,
                            Direction::Left => Direction::Up,
                        }
                    },
                    _ => { panic!() },
                };
                position = match direction {
                    Direction::Up => (position.0, position.1 - 1),
                    Direction::Right => (position.0 + 1, position.1),
                    Direction::Bottom => (position.0, position.1 + 1),
                    Direction::Left => (position.0 - 1, position.1),
                };
                min_x = min_x.min(position.0);
                min_y = min_y.min(position.1);
                max_x = max_x.max(position.0);
                max_y = max_y.max(position.1);

                mode = Mode::Color;
                inputs = vec![*colors.get(&position).unwrap_or(&0)];
            },
        };
        program = new_program;
    }

    println!("{}", count);

    min_x -= 10;
    min_y -= 10;
    max_x += 10;
    max_y += 10;

    let width = min_x.abs() + max_x.abs();
    let height = min_y.abs() + max_y.abs();

    let mut svg = String::new();
    svg.push_str(&format!("<svg viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\">", min_x, min_y, width, height));
    for y in (min_y - 1)..=max_y {
        for x in (min_x - 1)..=max_x {
            if *colors.get(&(x, y)).unwrap_or(&0) == 0 {
                svg.push_str(&format!("<rect fill=\"black\" width=\"1\" height=\"1\" x=\"{}\" y=\"{}\" />", x, y));
            } else if *colors.get(&(x, y)).unwrap_or(&0) == 1 {
                svg.push_str(&format!("<rect fill=\"white\" width=\"1\" height=\"1\" x=\"{}\" y=\"{}\" />", x, y));
            }
        }
    }
    svg.push_str("</svg>");
    fs::write("result.svg", svg).unwrap();
}


#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

#[derive(Debug)]
enum Mode {
    Color,
    Rotation,
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
enum ParamMode {
    Position,
    Immediate,
    Relative,
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
