use std::fs;
use std::collections::{HashMap,HashSet};

fn main() {
    println!("{}", compute_part_1("input.txt"));
    println!("{}", compute_part_2("input.txt"));
}

fn compute_part_1(filename: &str) -> i32 {
    let content = fs::read_to_string(filename).unwrap();
    let orbits = content.lines();
    let mut followers_by_object: HashMap<&str, HashSet<&str>> = HashMap::new();

    for orbit in orbits {
        let info: Vec<&str> = orbit.split(')').collect();
        let followers = followers_by_object.entry(info[0]).or_insert_with(HashSet::new);
        followers.insert(info[1]);
    }

    compute_part_1_recursive(&followers_by_object, "COM", 0)
}

fn compute_part_1_recursive(followers_by_object: &HashMap<&str, HashSet<&str>>, object: &str, level: i32) -> i32 {
    if let Some(followers) = followers_by_object.get(object) {
        let mut count = level;
        for follower in followers {
            count += compute_part_1_recursive(followers_by_object, follower, level + 1);
        }
        count
    } else {
        level
    }
}

fn compute_part_2(filename: &str) -> usize {
    let content = fs::read_to_string(filename).unwrap();
    let orbits_as_str = content.lines();
    let mut followers_by_object: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for orbit in orbits_as_str {
        let info: Vec<&str> = orbit.split(')').collect();

        let followers = followers_by_object.entry(info[0]).or_insert_with(HashSet::new);
        followers.insert(info[1]);

        orbits.insert(info[1], info[0]);
    }

    let mut my_parents: Vec<&str> = vec![];
    let mut parent = orbits.get("YOU").unwrap();
    loop {
        my_parents.push(parent);
        if let Some(new_parent) = orbits.get(parent) {
            parent = new_parent;
        } else {
            break;
        }
    }

    let mut current_parent = orbits.get("SAN").unwrap();
    let mut same_parent = None;
    let mut i = 1;
    while let Some(new_parent) = orbits.get(current_parent) {
        if let Some(index) = my_parents.iter().position(|x| x == new_parent) {
            same_parent = Some((new_parent, i + index));
            break;
        }
        i += 1;
        current_parent = new_parent;
    }

    if let Some((_, distance)) = same_parent {
        distance
    } else {
        panic!("oops");
    }

    // let end = orbits.get("SAN").unwrap();
    // let mut starts: Vec<&str> = vec![orbits.get("YOU").unwrap()];
    // for distance in 0..1000 {
    //     let mut new_starts: Vec<&str> = vec![];
    //     for start in &starts {
    //         if start == end {
    //             return distance;
    //         }
    //         if let Some(followers) = followers_by_object.get(start) {
    //             for follower in followers {
    //                 if !starts.contains(follower) {
    //                     new_starts.push(follower);
    //                 }
    //             }
    //         }
    //         if let Some(orbit) = orbits.get(start) {
    //             if !starts.contains(orbit) {
    //                 new_starts.push(orbit);
    //             }
    //         }
    //     }

    //     starts = new_starts;
    // }

    // panic!("Oops");
}



#[test]
fn part1() {
    assert_eq!(42, compute_part_1("test_part1.txt"));
}

#[test]
fn part2() {
    assert_eq!(4, compute_part_2("test_part2.txt"));
}