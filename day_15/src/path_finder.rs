use super::int_code::MachineResult;
use super::int_code::OpCodeMachine;

use std::collections::HashSet;
use std::rc::Rc;

struct Node {
    node_type: NodeType,
    location: Location,
    connections: [Option<Rc<Node>>; 4],
}

enum NodeType {
    Clear,
    Wall,
    Oxygen,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Location {
    x: i64,
    y: i64,
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

impl Node {
    pub fn new(location: Location, node_type: NodeType) -> Self {
        Self {
            node_type,
            location,
            connections: [None, None, None, None],
        }
    }
}

struct PathFinder {
    trail: Vec<Location>,
    machine: OpCodeMachine,
}

pub fn solve(initial_machine: OpCodeMachine) -> Vec<Location> {
    let start = PathFinder {
        trail: vec![Location { x: 0, y: 0 }],
        machine: initial_machine,
    };

    let mut active_paths = vec![start];
    let mut visited_locations = HashSet::<Location>::new();

    loop {
        let mut new_paths = Vec::<PathFinder>::new();
        for path in active_paths.drain(..) {
            let start_location = path.trail.last().unwrap();

            for connection in &[
                Movement::North,
                Movement::South,
                Movement::East,
                Movement::West,
            ] {
                let new_location = get_location(start_location, connection);

                if !visited_locations.insert(new_location.clone()) {
                    continue;
                }

                let mut new_machine = path.machine.clone();
                new_machine.input(*connection as i64);

                let move_result = match new_machine.run() {
                    MachineResult::Halt => panic!("The never ending, ended?"),
                    MachineResult::InputRequired => panic!("Machine wanted more input?"),
                    MachineResult::Output(value) => value.into(),
                };

                let is_on_oxygen = match move_result {
                    DroneStatusCodes::HitWall => continue,
                    DroneStatusCodes::Moved => false,
                    DroneStatusCodes::OnOxygen => true,
                };

                let mut new_trail = path.trail.clone();
                new_trail.push(new_location);

                if is_on_oxygen {
                    return new_trail;
                }

                new_paths.push(PathFinder {
                    trail: new_trail,
                    machine: new_machine,
                });
            }
        }

        if new_paths.len() == 0 {
            panic!("There is no end");
        }

        active_paths.extend(new_paths.drain(..));
    }
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
