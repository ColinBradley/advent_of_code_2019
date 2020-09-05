use crate::reactions::*;
use std::collections::HashMap;

mod reactions;

const INPUT: &'static str = include_str!("./input");

fn main() {
    let reactions_by_output_id = parse_reactions(INPUT).unwrap();

    let fuel_reaction = reactions_by_output_id.get(&Compound::Fuel).unwrap();

    let ore_required_count = get_ore_required_count(
        &reactions_by_output_id,
        &mut HashMap::new(),
        fuel_reaction,
        1,
    );

    println!(
        "{} units of ore required for 1 unit of fuel.",
        ore_required_count
    );

    let ore_count = 1_000_000_000_000;
    let fuel_count = get_fuel_from_ore(
        ore_count.clone(),
        &reactions_by_output_id,
        ore_required_count,
    );

    println!(
        "If I had {} units of ore, I'd buy you {} units of fuel, at ~{} units of ore to 1 fuel.",
        ore_count,
        fuel_count,
        ore_count as f64 / fuel_count as f64
    )
}
