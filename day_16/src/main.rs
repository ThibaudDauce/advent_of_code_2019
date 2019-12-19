fn main() {
    println!("{:?}", compute_part1(vec![0, 1, 0, -1], "59787832768373756387231168493208357132958685401595722881580547807942982606755215622050260150447434057354351694831693219006743316964757503791265077635087624100920933728566402553345683177887856750286696687049868280429551096246424753455988979991314240464573024671106349865911282028233691096263590173174821612903373057506657412723502892841355947605851392899875273008845072145252173808893257256280602945947694349746967468068181317115464342687490991674021875199960420015509224944411706393854801616653278719131946181597488270591684407220339023716074951397669948364079227701367746309535060821396127254992669346065361442252620041911746738651422249005412940728", 100));
}

fn compute_part1(pattern: Vec<i32>, input_as_str: &str, number_of_phases: i32) -> Vec<i32> {
    let mut inputs: Vec<i32> = vec![];
    for one_char in input_as_str.chars() {
        inputs.push(one_char.to_digit(10).unwrap() as i32);
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

fn get_multiplier(pattern: &Vec<i32>, input_element_i: usize, output_element_i: usize) -> i32 {
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