use super::int_code::*;
use std::collections::HashMap;

pub struct Game<F: Fn() -> i64> {
    screen: HashMap<Point, char>,
    pub machine: OpCodeMachine<F>,
    pub score: i64,
}

impl<F: Fn() -> i64> Game<F> {
    pub fn new(machine: OpCodeMachine<F>) -> Self {
        Game {
            screen: HashMap::new(),
            machine,
            score: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let y = if let Some(value) = self.machine.run() {
                value
            } else {
                break;
            };

            let x = if let Some(value) = self.machine.run() {
                value
            } else {
                break;
            };

            let tile_id = if let Some(value) = self.machine.run() {
                value
            } else {
                break;
            };

            if x < 0 || y < 0 {
                self.score = tile_id;
                continue;
            }

            self.screen.insert(
                Point {
                    x: x as u16,
                    y: y as u16,
                },
                tile_to_char(tile_id),
            );

            if tile_id == 4 {
                // Ball updated, release
                break;
            }
        }
    }

    pub fn draw(&self) -> String {
        let max_point = self.get_screen_dimensions();

        let mut output = String::with_capacity(
            (max_point.x as usize * max_point.y as usize) + max_point.y as usize,
        );

        for x in 0..=max_point.x {
            for y in 0..=max_point.y {
                output.push(self.get_char(&Point { x, y }).clone());
            }

            output.push('\n');
        }

        output
    }

    fn get_char(&self, location: &Point) -> &char {
        match self.screen.get(location) {
            Some(c) => c,
            _ => &' ',
        }
    }

    fn get_screen_dimensions(&self) -> Point {
        let mut height = 0;
        let mut width = 0;

        for point in self.screen.keys() {
            if point.x > width {
                width = point.x;
            }
            if point.y > height {
                height = point.y;
            }
        }

        Point {
            x: width,
            y: height,
        }
    }
}

fn tile_to_char(tile_id: i64) -> char {
    match tile_id {
        0 => ' ',
        1 => 'X',
        2 => '#',
        3 => '=',
        4 => 'O',
        _ => panic!("Unknown tile id"),
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}
