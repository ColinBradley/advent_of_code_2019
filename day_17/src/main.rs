mod int_code;

use int_code::*;
use std::collections::*;

const INPUT: &'static str = include_str!("./input.txt");

fn main() {
    let mut machine = OpCodeMachine::new(parse_code(INPUT));

    let mut scaffold_map = HashSet::<Point>::new();
    let mut points_to_check = Vec::<Point>::new();
    let mut biggest_x = 0;

    let mut current_consecutive_count = 0;
    let mut current_location = Point { x: 0, y: 0 };
    loop {
        match machine.run() {
            MachineResult::Output(v) => {
                match v {
                    35 | 60 | 62 | 94 | 118 => {
                        scaffold_map.insert(current_location.clone());
                        current_location.x += 1;
                        current_consecutive_count += 1;

                        if current_consecutive_count > 2 && current_location.y > 0 {
                            points_to_check.push(Point {
                                x: current_location.x - 1,
                                y: current_location.y,
                            });
                        }
                    }
                    46 => {
                        current_location.x += 1;

                        current_consecutive_count = 0;
                    }
                    10 => {
                        if current_location.x > biggest_x {
                            biggest_x = current_location.x;
                        }

                        current_location.x = 0;
                        current_location.y += 1;

                        current_consecutive_count = 0;
                    }
                    v => panic!("Unknown value {}", v),
                };
            }
            MachineResult::Halt => break,
            MachineResult::InputRequired => panic!("Input?"),
        }
    }

    let result: u32 = points_to_check
        .iter()
        .filter(|p| {
            scaffold_map.contains(&Point { x: p.x, y: p.y + 1 })
                && scaffold_map.contains(&Point { x: p.x, y: p.y - 1 })
        })
        .map(|p| p.x * p.y)
        .sum();

    for y in 0..current_location.y {
        for x in 0..=biggest_x {
            print!(
                "{}",
                if scaffold_map.contains(&Point { x, y }) {
                    "#"
                } else {
                    "."
                }
            );
        }
        print!("\n");
    }

    println!("Result: {}", result);
}

fn parse_code(value: &str) -> Vec<i64> {
    value
        .split(",")
        .map(|c| c.parse::<i64>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
