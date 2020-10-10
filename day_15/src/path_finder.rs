use super::int_code::MachineResult;
use super::int_code::OpCodeMachine;

use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Location {
    pub x: i64,
    pub y: i64,
}

#[derive(Copy, Clone)]
enum Movement {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

enum DroneStatusCodes {
    HitWall,
    Moved,
    OnOxygen,
}

impl From<i64> for DroneStatusCodes {
    fn from(value: i64) -> Self {
        match value {
            0 => DroneStatusCodes::HitWall,
            1 => DroneStatusCodes::Moved,
            2 => DroneStatusCodes::OnOxygen,
            _ => panic!("Unknown value"),
        }
    }
}

struct PathFinder {
    trail: Vec<Location>,
    machine: OpCodeMachine,
}

pub fn solve(
    initial_machine: OpCodeMachine,
) -> (
    Option<Vec<Location>>,
    Vec<Location>,
    HashMap<Location, char>,
) {
    let start = PathFinder {
        trail: vec![Location { x: 0, y: 0 }],
        machine: initial_machine,
    };

    let mut oxygen_path: Option<Vec<Location>> = None;
    let mut last_path = start.trail.clone();

    let mut active_paths = vec![start];
    let mut visited_locations = HashMap::<Location, char>::new();
    visited_locations.insert(Location { x: 0, y: 0 }, 'S');

    loop {
        let mut new_paths = Vec::<PathFinder>::new();
        for path in active_paths.drain(..) {
            let start_location = path.trail.last().unwrap();

            for direction in &[
                Movement::North,
                Movement::South,
                Movement::East,
                Movement::West,
            ] {
                let new_location = get_location(start_location, direction);

                if visited_locations.contains_key(&new_location) {
                    continue;
                }

                let mut new_machine = path.machine.clone();
                new_machine.input(*direction as i64);

                let move_result = match new_machine.run() {
                    MachineResult::Halt => panic!("The never ending, ended?"),
                    MachineResult::InputRequired => panic!("Machine wanted more input?"),
                    MachineResult::Output(value) => value.into(),
                };

                let is_on_oxygen = match move_result {
                    DroneStatusCodes::HitWall => {
                        visited_locations.insert(new_location, '#');
                        continue;
                    }
                    DroneStatusCodes::Moved => {
                        visited_locations.insert(new_location.clone(), '.');
                        false
                    }
                    DroneStatusCodes::OnOxygen => {
                        visited_locations.insert(new_location.clone(), 'O');
                        true
                    }
                };

                let mut new_trail = path.trail.clone();
                new_trail.push(new_location);

                if is_on_oxygen {
                    oxygen_path = Some(new_trail.clone());
                }

                last_path = new_trail.clone();

                new_paths.push(PathFinder {
                    trail: new_trail,
                    machine: new_machine,
                });
            }
        }

        if new_paths.len() == 0 {
            break;
        }

        active_paths.extend(new_paths.drain(..));
    }

    (oxygen_path, last_path, visited_locations)
}

const fn get_location(start: &Location, movement: &Movement) -> Location {
    match movement {
        Movement::North => Location {
            x: start.x,
            y: start.y + 1,
        },
        Movement::South => Location {
            x: start.x,
            y: start.y - 1,
        },
        Movement::East => Location {
            x: start.x + 1,
            y: start.y,
        },
        Movement::West => Location {
            x: start.x - 1,
            y: start.y,
        },
    }
}
