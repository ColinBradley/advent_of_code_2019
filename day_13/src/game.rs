use super::int_code::*;
use std::time::Duration;
use std::{collections::HashMap, thread};

pub struct Game {
    screen: HashMap<Point, char>,
    pub machine: OpCodeMachine,
    pub score: i64,
    ball_x: i64,
    paddle_x: i64,
}

enum GameTickResult {
    Halt,
    Draw,
    InputRequired,
}

impl Game {
    pub fn new(data: Vec<i64>) -> Self {
        Game {
            screen: HashMap::new(),
            machine: OpCodeMachine::new(data),
            score: 0,
            ball_x: 0,
            paddle_x: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.run_core() {
                GameTickResult::Halt => break,
                GameTickResult::InputRequired => {
                    self.machine.input(self.get_input());
                }
                GameTickResult::Draw => {
                    let screen = self.draw();

                    println!("{}", screen);
                    println!(
                        "Score: {}, Block count: {}",
                        self.score,
                        screen.chars().filter(|c| c == &'#').count()
                    );

                    thread::sleep(Duration::from_millis(10));
                }
            }
        }

        let screen = self.draw();

        println!("{}", screen);
        println!(
            "Score: {}, Block count: {}",
            self.score,
            screen.chars().filter(|c| c == &'#').count()
        );
    }

    fn run_core(&mut self) -> GameTickResult {
        loop {
            let x = match self.machine.run() {
                MachineResult::Output(value) => value,
                MachineResult::InputRequired => return GameTickResult::InputRequired,
                MachineResult::Halt => return GameTickResult::Halt,
            };

            let y = match self.machine.run() {
                MachineResult::Output(value) => value,
                MachineResult::InputRequired => return GameTickResult::InputRequired,
                MachineResult::Halt => return GameTickResult::Halt,
            };

            let tile_id = match self.machine.run() {
                MachineResult::Output(value) => value,
                MachineResult::InputRequired => return GameTickResult::InputRequired,
                MachineResult::Halt => return GameTickResult::Halt,
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

            match tile_id {
                3 => self.paddle_x = x,
                4 => self.ball_x = x,
                _ => continue,
            };

            return GameTickResult::Draw;
        }
    }

    pub fn draw(&self) -> String {
        let max_point = self.get_screen_dimensions();

        let mut output = String::with_capacity(
            (max_point.x as usize * max_point.y as usize) + max_point.y as usize,
        );

        for y in 0..=max_point.y {
            for x in 0..=max_point.x {
                output.push(self.get_char(&Point { x, y }).clone());
            }

            output.push('\n');
        }

        output
    }

    fn get_input(&self) -> i64 {
        (self.ball_x - self.paddle_x).signum()
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
