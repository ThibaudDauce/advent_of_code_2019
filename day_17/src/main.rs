use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
extern crate sdl2; 

use sdl2::pixels::Color;
use std::{thread, time};
use sdl2::rect::Rect;

#[derive(PartialEq, Debug)]
enum Type {
    Scaffold,
    OpenSpace,
    Robot,
}

#[allow(clippy::cognitive_complexity)]
fn main() {
    let program_as_string = fs::read_to_string("input.txt").unwrap();
    let mut program = parse_program(&program_as_string);

    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut map: HashMap<(i32, i32), Type> = HashMap::new();

    while let ProgramResult::Output(new_program, output) = run_program(program.clone(), vec![]) {
        program = new_program;

        println!("{} {} = {}", x, y, output);
        match output {
            10 => {
                y += 1;
                max_x = x.max(max_x);
                max_y = y.max(max_y);
                x = 0;
            }
            35 => {
                map.insert((x, y), Type::Scaffold);
                x += 1;
            },
            46 => {
                map.insert((x, y), Type::OpenSpace);
                x += 1;
            },
            94 => {
                map.insert((x, y), Type::Robot);
                x += 1;
            }
            _ => {
                println!("{}", output);
                panic!();
            },
        }
    }

    let mut sum = 0;
    println!("{} {}", max_x, max_y);
    for y in 0..max_y {
        for x in 0..max_x {
            let one_type = map.get(&(x, y)).unwrap_or(&Type::OpenSpace);
            match one_type {
                Type::Scaffold => print!("#"),
                Type::Robot    => print!("^"),
                Type::OpenSpace => print!("."),
            }
        }
        println!();
    }
    for y in 0..max_y {
        'blah: for x in 0..max_x {
            for position in &[(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                let one_type = map.get(position).unwrap_or(&Type::OpenSpace);

                match one_type {
                    Type::OpenSpace => continue 'blah,
                    _ => {},
                }
            }

            println!("{} {}", x, y);
            sum += x * y;
        }
        println!();
    }

    println!("Sum is {}", sum);

/*
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    
    loop {
        i += 1;
        let mut new_robots = HashMap::new();
        for ((x, y), program) in robots {
            for direction in 1..=4 {
                let new_position = match direction {
                    1 => (x - 1, y),
                    2 => (x + 1, y),
                    3 => (x, y - 1),
                    4 => (x, y + 1),
                    _ => panic!(),
                };

                if walls.contains(&new_position) || visited.contains(&new_position) {
                    continue;
                }

                if let ProgramResult::Output(new_program, output) = run_program(program.clone(), vec![direction]) {
                    visited.insert(new_position);
                    if output == 0 {
                        walls.insert(new_position);
                    } else if output == 1 {
                        new_robots.insert(new_position, new_program);
                    } else {
                        oxygen = Some(new_position);
                    }
                }
            }
        }
        if new_robots.is_empty() {
            break;
        }
        robots = new_robots;

        
        for y in -30..80 {
            for x in -30..80 {
                let color = if oxygen.is_some() && x == oxygen.unwrap().0 && y == oxygen.unwrap().1 {
                    Color::RGB(255, 0, 0)
                } else if x == 0 && y == 0 {
                    Color::RGB(0, 255, 0)
                } else if walls.contains(&(x, y)) {
                    Color::RGB(0, 0, 0)
                } else if visited.contains(&(x, y)) {
                    Color::RGB(200, 200, 200)
                } else {
                    Color::RGB(255, 255, 255)
                };

                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(((x + 30) * 10) as i32, ((y + 30) * 10) as i32, 10, 10)).unwrap();
            }
        }
        canvas.present();
    }

    println!("Found after {} steps", i);
    
    let mut oxygens = vec![oxygen.unwrap()];
    let mut oxygens_visited = vec![oxygen.unwrap()];
    i = 0;
    loop {
        i += 1;
        let mut new_oxygens = vec![];
        for (x, y) in oxygens {
            println!("{:?}", (x, y));
            for direction in 1..=4 {
                let new_position = match direction {
                    1 => (x - 1, y),
                    2 => (x + 1, y),
                    3 => (x, y - 1),
                    4 => (x, y + 1),
                    _ => panic!(),
                };

                println!("{:?}", new_position);

                if walls.contains(&new_position) || oxygens_visited.contains(&new_position) {
                    continue;
                }
                oxygens_visited.push(new_position);
                new_oxygens.push(new_position);
            }
        }
        if new_oxygens.is_empty() {
            break;
        }
        oxygens = new_oxygens;
        
        for y in -30..80 {
            for x in -30..80 {
                let color = if oxygen.is_some() && x == oxygen.unwrap().0 && y == oxygen.unwrap().1 {
                    Color::RGB(255, 0, 0)
                } else if walls.contains(&(x, y)) {
                    Color::RGB(0, 0, 0)
                } else if oxygens_visited.contains(&(x, y)) {
                    Color::RGB(0, 0, 255)
                } else if visited.contains(&(x, y)) {
                    Color::RGB(200, 200, 200)
                } else if x == 0 && y == 0 {
                    Color::RGB(0, 255, 0)
                } else {
                    Color::RGB(255, 255, 255)
                };

                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(((x + 30) * 10) as i32, ((y + 30) * 10) as i32, 10, 10)).unwrap();
            }
        }
        canvas.present();
    }

    println!("Oxygen full after {} steps", i);

    thread::sleep(time::Duration::from_millis(100 * 1000));
    */
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

#[derive(Debug, Clone)]
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
            panic!();
        }
        self.program.insert(self.get_index(mode), value);
        self.pointer += 1;
    }
    fn set_pointer(&mut self, value: i64) {
        self.pointer = value;
    }
    fn adjust_base(&mut self, value: i64) {
        self.base += value;
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
