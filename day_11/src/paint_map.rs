use std::collections::hash_map::HashMap;

pub enum Color {
    White,
    Black,
}

pub struct PaintMap {
    pub location: Position,
    direction: Heading,
    pub locations_to_color: HashMap<Position, Color>,
}

impl PaintMap {
    pub fn new() -> Self {
        PaintMap {
            direction: Heading::North,
            location: Position { x: 0, y: 0 },
            locations_to_color: HashMap::new(),
        }
    }

    pub fn drive(&mut self, direction: TurnDirection) {
        self.direction = self.direction.turn(&direction);
        self.location.transform(&self.direction);
    }

    pub fn set_color(&mut self, location: Position, color: Color) {
        self.locations_to_color.insert(location, color);
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.set_color(
            Position {
                x: self.location.x,
                y: self.location.y,
            },
            color,
        );
    }

    pub fn get_color(&self, location: &Position) -> &Color {
        match self.locations_to_color.get(location) {
            Some(color) => color,
            _ => &Color::Black,
        }
    }

    pub fn get_current_color(&self) -> &Color {
        self.get_color(&self.location)
    }

    pub fn to_ascii(&self) -> String {
        let mut lowest_x = i32::MAX;
        let mut highest_x = i32::MIN;
        let mut lowest_y = i32::MAX;
        let mut highest_y = i32::MIN;

        for location in self.locations_to_color.keys() {
            if location.x < lowest_x {
                lowest_x = location.x;
            } else if location.x > highest_x {
                highest_x = location.x;
            }

            if location.y < lowest_y {
                lowest_y = location.y;
            } else if location.y > highest_y {
                highest_y = location.y;
            }
        }

        let lines = highest_y - lowest_y + 1;
        let columns = highest_x - lowest_x + 1;

        let mut result = String::with_capacity(((lines * columns) + lines) as usize);

        for y in (lowest_y..=highest_y).rev() {
            for x in lowest_x..=highest_x {
                let color_char = match self.get_color(&Position { x, y }) {
                    Color::Black => '.',
                    Color::White => '#',
                };

                result.push(color_char);
            }
            result.push('\n');
        }

        result
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn transform(&mut self, heading: &Heading) {
        match heading {
            Heading::North => self.y = self.y + 1,
            Heading::East => self.x = self.x + 1,
            Heading::South => self.y = self.y - 1,
            Heading::West => self.x = self.x - 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Heading {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Heading {
    pub fn turn(&self, direction: &TurnDirection) -> Heading {
        match ((*self as u8) + (*direction as u8)) % 4 {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _other => panic!("Unknown direction {}", _other),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TurnDirection {
    Right = 1,
    // We want to wrap round a 4 heading system, so spin right 3 times
    // We don't want a negative value as we want to remain in positive values and not underflow etc
    Left = 3,
}

#[cfg(test)]
mod tests {

    mod heading {
        use super::super::*;

        #[test]
        fn north_turn_right() {
            assert_eq!(Heading::North.turn(&TurnDirection::Right), Heading::East);
        }

        #[test]
        fn north_turn_left() {
            assert_eq!(Heading::North.turn(&TurnDirection::Left), Heading::West);
        }

        #[test]
        fn west_turn_right() {
            assert_eq!(Heading::West.turn(&TurnDirection::Right), Heading::North);
        }
    }
}
