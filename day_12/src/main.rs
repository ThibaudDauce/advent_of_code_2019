fn main() {
    println!("{}", compute_part1(r#"
    <x=-4, y=-14, z=8>
    <x=1, y=-8, z=10>
    <x=-15, y=2, z=1>
    <x=-17, y=-17, z=16>
    "#, 1_000));

    println!("{}", compute_part2(r#"
    <x=-4, y=-14, z=8>
    <x=1, y=-8, z=10>
    <x=-15, y=2, z=1>
    <x=-17, y=-17, z=16>
    "#));
}

fn compute_gravity_diff(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> ((i64, i64, i64), (i64, i64, i64)) {
    let mut a_diff = (0, 0, 0);
    let mut b_diff = (0, 0, 0);

    if a.0 > b.0 {
        a_diff.0 = -1;
        b_diff.0 = 1;
    }
    if a.0 < b.0 {
        a_diff.0 = 1;
        b_diff.0 = -1;
    }

    if a.1 > b.1 {
        a_diff.1 = -1;
        b_diff.1 = 1;
    }
    if a.1 < b.1 {
        a_diff.1 = 1;
        b_diff.1 = -1;
    }

    if a.2 > b.2 {
        a_diff.2 = -1;
        b_diff.2 = 1;
    }
    if a.2 < b.2 {
        a_diff.2 = 1;
        b_diff.2 = -1;
    }

    (a_diff, b_diff)
}

fn add_vectors(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> (i64, i64, i64) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn compute_part1(positions_as_string: &str, steps: i64) -> i64 {
    let mut moons = vec![];
    let mut velocities = vec![];

    for position_as_string in positions_as_string.trim().lines() {
        let axis: Vec<&str> = position_as_string.trim().trim_start_matches('<').trim_end_matches('>').split(", ").collect();
        let x: i64 = axis[0].split('=').collect::<Vec<&str>>()[1].parse().unwrap();
        let y: i64 = axis[1].split('=').collect::<Vec<&str>>()[1].parse().unwrap();
        let z: i64 = axis[2].split('=').collect::<Vec<&str>>()[1].parse().unwrap();
        moons.push((x, y, z));
        velocities.push((0, 0, 0));
    }

    for _ in 1..=steps {
        for (moon_index, moon) in moons.iter().enumerate() {
            for (other_moon_index, other_moon) in moons.iter().enumerate() {
                if moon_index >= other_moon_index {
                    continue;
                }

                let (moon_diff, other_moon_diff) = compute_gravity_diff(moon, other_moon);
                velocities[moon_index] = add_vectors(&velocities[moon_index], &moon_diff);
                velocities[other_moon_index] = add_vectors(&velocities[other_moon_index], &other_moon_diff);
            }
        }
        for i in 0..4 {
            moons[i] = add_vectors(&moons[i], &velocities[i]);
        }
    }

    let mut sum = 0;
    for i in 0..4 {
        let pot = moons[i].0.abs() + moons[i].1.abs() + moons[i].2.abs();
        let kin = velocities[i].0.abs() + velocities[i].1.abs() + velocities[i].2.abs();
        sum += pot * kin;
    }
    sum
}

fn compute_part2(positions_as_string: &str) -> i64 {
    // x = 0
    // y = 1
    // z = 2
    let mut moons = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let initial_velocities = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
    let mut velocities = initial_velocities.clone();

    for (i, position_as_string) in positions_as_string.trim().lines().enumerate() {
        let axis: Vec<&str> = position_as_string.trim().trim_start_matches('<').trim_end_matches('>').split(", ").collect();
        let x: i64 = axis[0].split('=').collect::<Vec<&str>>()[1].parse().unwrap();
        let y: i64 = axis[1].split('=').collect::<Vec<&str>>()[1].parse().unwrap();
        let z: i64 = axis[2].split('=').collect::<Vec<&str>>()[1].parse().unwrap();
        moons[0][i] = x;
        moons[1][i] = y;
        moons[2][i] = z;
    }

    let initial_moons = moons.clone();

    let mut axis_steps = [0, 0, 0];
    for axis in 0..3 {
        let mut steps = 0;
        loop {
            steps += 1;
            for (moon_index, moon_position) in moons[axis].iter().enumerate() {
                for (other_moon_index, other_position) in moons[axis].iter().enumerate() {
                    if moon_index >= other_moon_index {
                        continue;
                    }
    
                    if moon_position > other_position {
                        velocities[axis][moon_index] -= 1;
                        velocities[axis][other_moon_index] += 1;
                    }
                    if moon_position < other_position {
                        velocities[axis][moon_index] += 1;
                        velocities[axis][other_moon_index] -= 1;
                    }
                }
            }

            for i in 0..4 {
                moons[axis][i] += velocities[axis][i];
            }

            if moons[axis] == initial_moons[axis] && velocities[axis] == initial_velocities[axis] {
                break;
            }
        }
        axis_steps[axis] = steps;
    }

    println!("{:?}", axis_steps);

    let mut multiplier = [1, 1, 1];
    let mut results = [
        multiplier[0] * axis_steps[0],
        multiplier[1] * axis_steps[1],
        multiplier[2] * axis_steps[2],
    ];
    loop {
        if results[0] == results[1] && results[1] == results[2] {
            return results[0];
        }

        let min_index = results.iter().enumerate().min_by(|&(_, x), &(_, y)| x.cmp(y)).unwrap().0;
        multiplier[min_index as usize] += 1;
        results[min_index as usize] += axis_steps[min_index as usize];
    }
}

#[test]
fn part1() {
    assert_eq!(179, compute_part1(r#"
    <x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>
    "#, 10));
    assert_eq!(1940, compute_part1(r#"
    <x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>
    "#, 100));
}


#[test]
fn part2() {
    assert_eq!(2772, compute_part2(r#"
    <x=-1, y=0, z=2>
    <x=2, y=-10, z=-7>
    <x=4, y=-8, z=8>
    <x=3, y=5, z=-1>
    "#));
    assert_eq!(4_686_774_924, compute_part2(r#"
    <x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>
    "#));
}