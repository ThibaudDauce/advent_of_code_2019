use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let reactions = parse_input(&input);

    println!("{}", compute_part1(&reactions, HashMap::new()).0);
    println!("{}", compute_part2(&input));
}

#[derive(Debug)]
struct Reaction {
    inputs: HashMap<String, i64>,
    output: i64,
}

fn compute_part2(input: &str) -> i64 {
    let mut fuel = 0;
    let mut ore = 1_000_000_000_000;
    let mut stock = HashMap::new();

    let reactions = parse_input(input);

    loop {
        let (ore_needed, new_stock) = compute_part1(&reactions, stock);
        if ore_needed > ore {
            break;
        }
        ore -= ore_needed;
        fuel += 1;
        stock = new_stock;
    }

    fuel
}

fn parse_input(input: &str) -> HashMap<String, Reaction> {
    let mut reactions: HashMap<String, Reaction> = HashMap::new();

    for reaction_as_string in input.trim().lines() {
        let reaction_as_vec: Vec<&str> = reaction_as_string.trim().split(" => ").collect();
        let input_as_string = reaction_as_vec[0];
        let output_as_string = reaction_as_vec[1];

        let output_as_vec: Vec<&str> = output_as_string.split(' ').collect();
        let mut reaction = Reaction { inputs: HashMap::new(), output: output_as_vec[0].parse().unwrap() };

        for ingredient_as_string in input_as_string.split(", ") {
            let ingredient_as_vec: Vec<&str> = ingredient_as_string.split(' ').collect();
            reaction.inputs.insert(ingredient_as_vec[1].to_string(), ingredient_as_vec[0].parse().unwrap());
        }

        reactions.insert(output_as_vec[1].to_string(), reaction);
    }

    reactions
}

fn compute_part1(reactions: &HashMap<String, Reaction>, mut stock: HashMap<String, i64>) -> (i64, HashMap<String, i64>) {
    let mut inputs = reactions.get("FUEL").unwrap().inputs.clone();
    loop {
        // println!("\n\n####################\n\n");
        // println!("Stock {:?}", stock);

        // println!("{:?}", inputs);
        let mut done = true;
        let mut new_inputs = inputs.clone();
        for (ingredient, quantity) in &inputs {
            if ingredient == "ORE" || *quantity == 0 {
                continue;
            }
            done = false;

            let reaction = reactions.get(ingredient).unwrap();

            let mut quantity_needed = *quantity;
            if let Some(quantity_in_stock) = stock.remove(ingredient) {
                // println!("!!! {} in stock {} !!!", ingredient, quantity_in_stock);

                if quantity_in_stock > quantity_needed {
                    stock.insert(ingredient.clone(), quantity_in_stock - quantity_needed);
                    quantity_needed = 0;
                } else {
                    quantity_needed -= quantity_in_stock;
                }
            }
            let mut number_of_reactions = quantity_needed / reaction.output;
            if quantity_needed % reaction.output > 0 {
                number_of_reactions += 1;
            }

            // println!("Ingredient {} / Quantity {} / Quantity needed {} / Number of reactions {}", ingredient, *quantity, quantity_needed, number_of_reactions);

            {
                let entry = new_inputs.entry(ingredient.clone()).or_insert(0);
                assert!(*entry >= *quantity);
                if *entry == *quantity {
                    new_inputs.remove(&ingredient.clone());
                } else {
                    *entry -= *quantity;
                }
            }
            
            if number_of_reactions > 0 {
                for (new_input, new_quantity) in &reaction.inputs {
                    let entry = new_inputs.entry(new_input.clone()).or_insert(0);
                    *entry += new_quantity * number_of_reactions;
                }
            }

            let remaining = (reaction.output * number_of_reactions) - quantity_needed;
            if remaining > 0 {
                // println!("Putting {}x{} in stock", remaining, ingredient);
                let entry = stock.entry(ingredient.clone()).or_insert(0);
                *entry += remaining;
            }
            assert!(remaining >= 0);
            // println!("New stock {:?}", stock);
            // println!("New inputs {:?}", new_inputs);
        }

        if done {
            break;
        }
        inputs = new_inputs;
    }

    (*inputs.get("ORE").unwrap(), stock)
}

#[test]
fn part1() {
    assert_eq!(165, compute_part1(&parse_input(r#"
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    "#), HashMap::new()).0);
    assert_eq!(13312, compute_part1(&parse_input(r#"
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    "#), HashMap::new()).0);
    assert_eq!(180697, compute_part1(&parse_input(r#"
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF
    "#), HashMap::new()).0);
    assert_eq!(2210736, compute_part1(&parse_input(r#"
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX
    "#), HashMap::new()).0);
}

#[test]
fn part2() {
    assert_eq!(82892753, compute_part2(r#"
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    "#));
    assert_eq!(5586022, compute_part2(r#"
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF
    "#));
    assert_eq!(460664, compute_part2(r#"
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX
    "#));
}