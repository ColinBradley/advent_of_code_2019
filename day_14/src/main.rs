use crate::reactions::*;
use std::collections::HashMap;

mod reactions;

const INPUT: &'static str = include_str!("./input");

fn main() {
    let reactions_by_output_id = parse_reactions(INPUT).unwrap();

    let fuel_reaction = reactions_by_output_id.get(&Compound::Fuel).unwrap();

    println!(
        "Ore required: {}",
        get_ore_required_count(
            &reactions_by_output_id,
            &mut HashMap::new(),
            fuel_reaction,
            1,
        )
    );
}
