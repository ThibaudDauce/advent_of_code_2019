fn main() {
    // println!("{:?}", compute_part1(vec![0, 1, 0, -1], "59787832768373756387231168493208357132958685401595722881580547807942982606755215622050260150447434057354351694831693219006743316964757503791265077635087624100920933728566402553345683177887856750286696687049868280429551096246424753455988979991314240464573024671106349865911282028233691096263590173174821612903373057506657412723502892841355947605851392899875273008845072145252173808893257256280602945947694349746967468068181317115464342687490991674021875199960420015509224944411706393854801616653278719131946181597488270591684407220339023716074951397669948364079227701367746309535060821396127254992669346065361442252620041911746738651422249005412940728", 100));

    println!("{:?}", compute_part2(vec![0, 1, 0, -1], "59787832768373756387231168493208357132958685401595722881580547807942982606755215622050260150447434057354351694831693219006743316964757503791265077635087624100920933728566402553345683177887856750286696687049868280429551096246424753455988979991314240464573024671106349865911282028233691096263590173174821612903373057506657412723502892841355947605851392899875273008845072145252173808893257256280602945947694349746967468068181317115464342687490991674021875199960420015509224944411706393854801616653278719131946181597488270591684407220339023716074951397669948364079227701367746309535060821396127254992669346065361442252620041911746738651422249005412940728", 100));
}

fn compute_part1(pattern: Vec<i64>, input_as_str: &str, number_of_phases: i64) -> Vec<i64> {
    let mut inputs: Vec<i64> = vec![];
    for one_char in input_as_str.chars() {
        inputs.push(one_char.to_digit(10).unwrap() as i64);
    }

    for phase_i in 0..number_of_phases {
        let mut outputs = vec![];
        for output_element_i in 0..inputs.len() {
            let mut sum = 0;

            for (input_element_i, input) in inputs.iter().enumerate() {
                // print!("{}*{} + ", input, get_multiplier(&pattern, input_element_i, output_element_i));
                sum += input * get_multiplier(&pattern, input_element_i, output_element_i);
            }
            // println!("= {}", (sum % 10).abs());
            outputs.push((sum % 10).abs());
        }
        // println!("After phase {} {:?}", phase_i + 1, outputs);
        inputs = outputs;
    }

    inputs
}

enum NextMode {
    Add,
    Sub,
}

enum Mode {
    InitialZero,
    Add,
    Sub,
    Zero(NextMode),
}

fn compute_part2(pattern: Vec<i64>, input_as_str: &str, number_of_phases: i64) -> Vec<i64> {
    let mut inputs: Vec<i64> = vec![];
    for one_char in input_as_str.chars() {
        inputs.push(one_char.to_digit(10).unwrap() as i64);
    }
    let offset = get_offset(&inputs);
    inputs = inputs.iter().cycle().take(inputs.len() * 10_000).copied().collect();

    for phase_i in 0..number_of_phases {
        println!("Phase {}", phase_i);
        let mut outputs = Vec::with_capacity(inputs.len());
        let mut cum_sum = Vec::with_capacity(inputs.len());
        cum_sum.push(inputs[0]);
        for i in 1..inputs.len() {
            cum_sum.push(cum_sum[i - 1] + inputs[i]);
        }


        for output_element_i in 0..inputs.len() {
            let mut sum: i64 = 0;
            let mut input_element_i = 0;
            let mut mode = Mode::InitialZero;

            while input_element_i < inputs.len() {
                match mode {
                    Mode::InitialZero => {
                        input_element_i += output_element_i;
                        mode = Mode::Add;
                    },
                    Mode::Add => {
                        let previous_sum = if input_element_i == 0 {
                            0
                        } else {
                            cum_sum[input_element_i - 1]
                        };
                        sum += cum_sum[(input_element_i + output_element_i).min(inputs.len() - 1)] - previous_sum;
                        input_element_i += output_element_i + 1;
                        mode = Mode::Zero(NextMode::Sub);
                    },
                    Mode::Sub => {
                        let previous_sum = if input_element_i == 0 {
                            0
                        } else {
                            cum_sum[input_element_i - 1]
                        };
                        sum -= cum_sum[(input_element_i + output_element_i).min(inputs.len() - 1)] - previous_sum;
                        input_element_i += output_element_i + 1;
                        mode = Mode::Zero(NextMode::Add);
                    },
                    Mode::Zero(next_mode) => {
                        input_element_i += output_element_i + 1;
                        mode = match next_mode {
                            NextMode::Add => Mode::Add,
                            NextMode::Sub => Mode::Sub,
                        };
                    },
                };
            }

            outputs.push((sum % 10).abs());
        }
        // println!("After phase {} {:?}", phase_i + 1, outputs);
        inputs = outputs;
    }

    inputs[offset..(offset+8)].to_vec()
}

fn get_offset(digits: &Vec<i64>) -> usize {
    let mut result = 0;
    let base: i64 = 10;
    for i in 0..7 {
        result += digits[i] * base.pow(6 - (i as u32));
    }

    result as usize
}

fn get_multiplier(pattern: &Vec<i64>, input_element_i: usize, output_element_i: usize) -> i64 {
    pattern[((input_element_i + 1) / (output_element_i + 1)) % pattern.len()]
}

#[test]
fn part1() {
    assert_eq!(1, get_multiplier(&vec![0, 1, 0, -1], 0, 0));
    assert_eq!(0, get_multiplier(&vec![0, 1, 0, -1], 1, 0));
    assert_eq!(-1, get_multiplier(&vec![0, 1, 0, -1], 2, 0));
    assert_eq!(0, get_multiplier(&vec![0, 1, 0, -1], 3, 0));
    assert_eq!(0, get_multiplier(&vec![0, 1, 0, -1], 7, 0));

    assert_eq!(0, get_multiplier(&vec![0, 1, 0, -1], 0, 2));
    assert_eq!(0, get_multiplier(&vec![0, 1, 0, -1], 1, 2));
    assert_eq!(1, get_multiplier(&vec![0, 1, 0, -1], 2, 2));
    assert_eq!(1, get_multiplier(&vec![0, 1, 0, -1], 3, 2));
    assert_eq!(0, get_multiplier(&vec![0, 1, 0, -1], 7, 2));

    assert_eq!(1, get_multiplier(&vec![0, 1, 0, -1], 7, 7));

    assert_eq!(vec![0,1,0,2,9,4,9,8], compute_part1(vec![0, 1, 0, -1], "12345678", 4));
    assert_eq!(vec![2,4,1,7,6,1,7,6][..], compute_part1(vec![0, 1, 0, -1], "80871224585914546619083218645595", 100)[0..8]);
    assert_eq!(vec![7,3,7,4,5,4,1,8][..], compute_part1(vec![0, 1, 0, -1], "19617804207202209144916044189917", 100)[0..8]);
    assert_eq!(vec![5,2,4,3,2,1,3,3][..], compute_part1(vec![0, 1, 0, -1], "69317163492948606335995924319873", 100)[0..8]);
}

#[test]
fn part2() {
    assert_eq!(303673, get_offset(&vec![0,3,0,3,6,7,3,2,5,7]));

    assert_eq!(vec![8,4,4,6,2,0,2,6], compute_part2(vec![0, 1, 0, -1], "03036732577212944063491565474664", 100));
    assert_eq!(vec![7,8,7,2,5,2,7,0], compute_part2(vec![0, 1, 0, -1], "02935109699940807407585447034323", 100));
    assert_eq!(vec![5,3,5,5,3,7,3,1], compute_part2(vec![0, 1, 0, -1], "03081770884921959731165446850517", 100));
}