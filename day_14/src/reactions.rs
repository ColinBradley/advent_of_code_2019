use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Reaction {
    pub output: ReactionChemical,
    pub inputs: Vec<ReactionChemical>,
}

pub struct ReactionChemical {
    pub id: Compound,
    pub quantity: u32,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Compound {
    Ore,
    Fuel,
    Other(String),
}

#[derive(Debug)]
pub enum ChemicalParseError {
    LineError(String),
    QuantityError(ParseIntError),
}

impl FromStr for ReactionChemical {
    type Err = ChemicalParseError;

    fn from_str(source: &str) -> Result<Self, <Self as FromStr>::Err> {
        match source.split(" ").collect::<Vec<&str>>().as_slice() {
            [left, right] => Ok(Self {
                id: match *right {
                    "ORE" => Compound::Ore,
                    "FUEL" => Compound::Fuel,
                    right => Compound::Other(String::from(right)),
                },
                quantity: left
                    .parse::<u32>()
                    .or_else(|e| Err(ChemicalParseError::QuantityError(e)))?,
            }),
            _ => Err(ChemicalParseError::LineError(String::from(source))),
        }
    }
}

pub fn parse_reactions(source: &str) -> Result<HashMap<Compound, Reaction>, ChemicalParseError> {
    let lines: Vec<_> = source.lines().collect();
    let mut result = HashMap::with_capacity(lines.len());

    for line in lines {
        let (inputs_source, output_source) =
            match line.split("=>").collect::<Vec<&str>>().as_slice() {
                [left, right] => (left.trim(), right.trim()),
                _ => return Err(ChemicalParseError::LineError(String::from(line))),
            };

        let reaction = Reaction {
            output: output_source.parse()?,
            inputs: inputs_source
                .split(",")
                .map(|c| c.trim().parse::<ReactionChemical>())
                .collect::<Result<Vec<_>, _>>()?,
        };

        result.insert(reaction.output.id.clone(), reaction);
    }

    Ok(result)
}

pub fn get_ore_required_count(
    reactions_by_output_id: &HashMap<Compound, Reaction>,
    compound_cache: &mut HashMap<Compound, u32>,
    reaction: &Reaction,
    amount_required: u32,
) -> u32 {
    let cache_amount = compound_cache
        .entry(reaction.output.id.clone())
        .or_insert(0);

    if &amount_required <= cache_amount {
        // Use up some/all cache
        *cache_amount -= amount_required;

        return 0;
    }

    let amount_required = amount_required - *cache_amount;
    *cache_amount = 0;

    let quantity_adjust = (amount_required as f64 / reaction.output.quantity as f64).ceil() as u32;

    let excess = (reaction.output.quantity * quantity_adjust) as i32 - amount_required as i32;
    if excess > 0 {
        *cache_amount = excess as u32;
    }

    if let [ReactionChemical {
        id: Compound::Ore,
        quantity,
    }] = reaction.inputs.as_slice()
    {
        return quantity_adjust * quantity;
    }

    // Resolve inputs

    reaction
        .inputs
        .iter()
        .map(|c| {
            let reaction = reactions_by_output_id.get(&c.id).unwrap();
            let ore = get_ore_required_count(
                reactions_by_output_id,
                compound_cache,
                reaction,
                c.quantity * quantity_adjust,
            );

            ore + 0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        test_core(include_str!("./test_data/1"), 31);
    }

    #[test]
    fn example_2() {
        test_core(include_str!("./test_data/2"), 165);
    }

    #[test]
    fn example_3() {
        test_core(include_str!("./test_data/3"), 13312);
    }

    #[test]
    fn example_4() {
        test_core(include_str!("./test_data/4"), 180697);
    }

    #[test]
    fn example_5() {
        test_core(include_str!("./test_data/5"), 2210736);
    }

    fn test_core(source: &str, expected_ore_count: u32) {
        let reactions = parse_reactions(source).unwrap();
        let fuel = reactions.get(&Compound::Fuel).unwrap();
        let result = get_ore_required_count(&reactions, &mut HashMap::new(), fuel, 1);

        assert_eq!(result, expected_ore_count);
    }
}
