use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", compute_part1(&input));

    println!("{:?}", compute_part2(&input, &(25.0, 31.0)));
}

fn get_direction(a: &(f32, f32), b: &(f32, f32)) -> (f32, f32) {
    ((a.0 - b.0), (a.1 - b.1))
}

fn get_vector_length(a: &(f32, f32)) -> f32 {
    (a.0 * a.0 + a.1 * a.1).sqrt()
}

fn get_unit_vector(a: &(f32, f32)) -> (f32, f32) {
    let length = get_vector_length(a);

    (a.0 / length, a.1 / length)
}

fn is_equal_with_epsilon(a: &(f32, f32), b: &(f32, f32)) -> bool {
    (a.0 - b.0).abs() < 0.0001 && (a.1 - b.1).abs() < 0.0001  
}

fn get_score(a: &(f32, f32)) -> f32 {
    let mut score = a.1.atan2(a.0) + 3.14 / 2.0 - std::f32::consts::PI;

    if score < 0.0 {
        score += 2.0 * std::f32::consts::PI;
    }

    score
}

fn compute_part1(map_as_string: &str) -> ((f32, f32), i32) {
    let mut asteroids: Vec<(f32, f32)> = vec![];
    for (y, line) in map_as_string.trim().lines().enumerate() {
        for (x, one_char) in line.trim().chars().enumerate() {
            if one_char == '#' {
                asteroids.push((x as f32, y as f32));
            }
        }
    }

    let mut result = ((0.0, 0.0), 0);
    for asteroid in &asteroids {
        let mut count = 0;
        for asteroid_to_see in &asteroids {
            if asteroid == asteroid_to_see {
                continue;
            }

            let mut blocked = false;
            let view = get_direction(asteroid, asteroid_to_see);
            let view_length = get_vector_length(&view);
            for asteroid_between in &asteroids {
                if asteroid_between == asteroid_to_see {
                    continue;
                }
                if asteroid_between == asteroid {
                    continue;
                }
                let interception = get_direction(asteroid, asteroid_between);
                let interception_length = get_vector_length(&interception);
                if view_length > interception_length && is_equal_with_epsilon(&get_unit_vector(&view), &get_unit_vector(&interception)) {
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                count += 1;
            }
        }

        if count > result.1 {
            result = (*asteroid, count);
        }
    }

    result
}

fn compute_part2(map_as_string: &str, position: &(f32, f32)) -> f32 {
    let mut asteroids: Vec<(f32, f32)> = vec![];
    for (y, line) in map_as_string.trim().lines().enumerate() {
        for (x, one_char) in line.trim().chars().enumerate() {
            if one_char == '#' {
                asteroids.push((x as f32, y as f32));
            }
        }
    }

    let mut list_to_destroy = vec![];
    for asteroid in &asteroids {
        if position != asteroid {
            continue;
        }
        for asteroid_to_see in &asteroids {
            if asteroid == asteroid_to_see {
                continue;
            }

            let mut blocked = false;
            let view = get_direction(asteroid, asteroid_to_see);
            let view_length = get_vector_length(&view);
            for asteroid_between in &asteroids {
                if asteroid_between == asteroid_to_see {
                    continue;
                }
                if asteroid_between == asteroid {
                    continue;
                }
                let interception = get_direction(asteroid, asteroid_between);
                let interception_length = get_vector_length(&interception);
                if view_length > interception_length && is_equal_with_epsilon(&get_unit_vector(&view), &get_unit_vector(&interception)) {
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                list_to_destroy.push((get_score(&view), asteroid_to_see));
            }
        }
    }

    list_to_destroy.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("{:?}", list_to_destroy);
    let asteroid_200 = list_to_destroy[200 - 1].1;
    asteroid_200.0 * 100.0 + asteroid_200.1
}

#[test]
fn part1() {
    assert_eq!(((3.0, 4.0), 8), compute_part1(r###"
    .#..#
    .....
    #####
    ....#
    ...##
"###));
    assert_eq!(((5.0, 8.0), 33), compute_part1(r###"
    ......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####
"###));
    assert_eq!(((1.0, 2.0), 35), compute_part1(r###"
    #.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.
"###));
    assert_eq!(((6.0, 3.0), 41), compute_part1(r###"
    .#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..
"###));
    assert_eq!(((11.0, 13.0), 210), compute_part1(r###"
    .#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##
"###));
}

#[test]
fn part2() {
    assert_eq!(802.0, compute_part2(r###"
    .#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##
"###, &(11.0, 13.0)));
}