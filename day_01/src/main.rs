use std::fs;

fn main() {
    let mut total = 0;
    let mut total_part2 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        total += compute_part1(line.parse().unwrap());
        total_part2 += compute_part2(line.parse().unwrap());
    }

    println!("{}", total);
    println!("{}", total_part2);
}

fn compute_part1(x: i32) -> i32 {
    (x / 3) - 2
}

fn compute_part2(mass: i32) -> i32 {
    let mut total = 0;
    let mut fuel = compute_part1(mass);
    while fuel > 0 {
        total += fuel;
        fuel = compute_part1(fuel);
    }

    total
}

#[test]
fn part1() {
    assert_eq!(2, compute_part1(12));
    assert_eq!(2, compute_part1(14));
    assert_eq!(654, compute_part1(1969));
    assert_eq!(33583, compute_part1(100_756));
}

#[test]
fn part2() {
    assert_eq!(2, compute_part2(14));
    assert_eq!(966, compute_part2(1969));
    assert_eq!(50346, compute_part2(100_756));
}