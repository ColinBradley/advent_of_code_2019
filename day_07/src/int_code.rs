pub struct OpCodeMachine {
    data: Vec<i32>,
    inputs: Vec<i32>,
    pointer: usize,
    pub is_complete: bool,
}

impl OpCodeMachine {
    pub fn new(data: Vec<i32>) -> OpCodeMachine {
        OpCodeMachine {
            data,
            inputs: Vec::new(),
            pointer: 0,
            is_complete: false,
        }
    }

    pub fn with_input(&mut self, input: i32) -> &mut OpCodeMachine {
        self.inputs.push(input);
        self
    }

    pub fn run(&mut self) -> Option<i32> {
        if self.is_complete {
            return None;
        }

        loop {
            let op = self.data.get(self.pointer).map(|v| get_op(v)).unwrap();

            match op {
                OpCode::Add(p1_mode, p2_mode) => {
                    let p1 = get_value(&self.data, self.pointer + 1, p1_mode);
                    let p2 = get_value(&self.data, self.pointer + 2, p2_mode);
                    let p3 = self.data[self.pointer + 3];
                    self.data[p3 as usize] = p1 + p2;
                    self.pointer += 4;
                }
                OpCode::Multiply(p1_mode, p2_mode) => {
                    let p1 = get_value(&self.data, self.pointer + 1, p1_mode);
                    let p2 = get_value(&self.data, self.pointer + 2, p2_mode);
                    let p3 = self.data[self.pointer + 3];
                    self.data[p3 as usize] = p1 * p2;
                    self.pointer += 4;
                }
                OpCode::Input() => {
                    let address = self.data[self.pointer + 1] as usize;
                    self.data[address] = self.inputs.remove(0);
                    self.pointer += 2;
                }
                OpCode::Output(p1_mode) => {
                    let value = get_value(&self.data, self.pointer + 1, p1_mode).clone();
                    self.pointer += 2;
                    return Some(value.clone());
                }
                OpCode::JumpIfTrue(p1_mode, p2_mode) => {
                    let p1 = get_value(&self.data, self.pointer + 1, p1_mode);
                    if *p1 == 0 {
                        self.pointer += 3;
                    } else {
                        self.pointer = *get_value(&self.data, self.pointer + 2, p2_mode) as usize;
                    }
                }
                OpCode::JumpIfFalse(p1_mode, p2_mode) => {
                    let p1 = get_value(&self.data, self.pointer + 1, p1_mode);
                    if *p1 == 0 {
                        self.pointer = *get_value(&self.data, self.pointer + 2, p2_mode) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                OpCode::LessThan(p1_mode, p2_mode) => {
                    let p1 = get_value(&self.data, self.pointer + 1, p1_mode);
                    let p2 = get_value(&self.data, self.pointer + 2, p2_mode);
                    let p3 =
                        *get_value(&self.data, self.pointer + 3, ParameterMode::Immediate) as usize;

                    self.data[p3] = if p1 < p2 { 1 } else { 0 };
                    self.pointer += 4;
                }
                OpCode::Equals(p1_mode, p2_mode) => {
                    let p1 = get_value(&self.data, self.pointer + 1, p1_mode);
                    let p2 = get_value(&self.data, self.pointer + 2, p2_mode);
                    let p3 =
                        *get_value(&self.data, self.pointer + 3, ParameterMode::Immediate) as usize;

                    self.data[p3] = if p1 == p2 { 1 } else { 0 };
                    self.pointer += 4;
                }
                OpCode::Halt => {
                    self.is_complete = true;
                    return None;
                }
            };
        }
    }
}

fn get_value<'a>(data: &'a Vec<i32>, location: usize, mode: ParameterMode) -> &'a i32 {
    match mode {
        ParameterMode::Immediate => data.get(location).unwrap(),
        ParameterMode::Position => data.get(*data.get(location).unwrap() as usize).unwrap(),
    }
}

fn get_op(value: &i32) -> OpCode {
    let ((p1, p2, _), operation) = parse_operation(value);

    match operation {
        1 => OpCode::Add(p1.into(), p2.into()),
        2 => OpCode::Multiply(p1.into(), p2.into()),
        3 => OpCode::Input(),
        4 => OpCode::Output(p1.into()),
        5 => OpCode::JumpIfTrue(p1.into(), p2.into()),
        6 => OpCode::JumpIfFalse(p1.into(), p2.into()),
        7 => OpCode::LessThan(p1.into(), p2.into()),
        8 => OpCode::Equals(p1.into(), p2.into()),
        99 => OpCode::Halt,
        _ => panic!("unknown code"),
    }
}

fn parse_operation(value: &i32) -> ((i32, i32, i32), i32) {
    let operation = value % 100;

    let p1 = value / 100 % 10;
    let p2 = value / 1_000 % 10;
    let p3 = value / 10_000 % 10;

    ((p1, p2, p3), operation)
}

enum OpCode {
    Add(ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode),
    Input(),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
    Halt,
}
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(value: i32) -> ParameterMode {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("unknown code"),
        }
    }
}

#[cfg(test)]
mod tests {

    mod get_digits {
        use super::super::*;

        #[test]
        fn basic() {
            assert_eq!(parse_operation(&12345), ((3, 2, 1), 45));
        }
    }

    mod run {
        use super::super::*;

        const EQUAL_TO_8_DATA_POSITION: &'static [i32] = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        #[test]
        fn example_equal_8_position_mode_false() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_POSITION.to_vec());
            machine.with_input(5);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn example_equal_8_position_mode_true() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_POSITION.to_vec());
            machine.with_input(8);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        const EQUAL_TO_8_DATA_IMMEDIATE: &'static [i32] = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];

        #[test]
        fn example_equal_8_immediate_mode_false() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_IMMEDIATE.to_vec());
            machine.with_input(5);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn example_equal_8_immediate_mode_true() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_IMMEDIATE.to_vec());
            machine.with_input(8);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        const LESS_THAN_8_DATA_POSITION: &'static [i32] = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        #[test]
        fn less_than_8_position_mode_false() {
            let mut machine = OpCodeMachine::new(LESS_THAN_8_DATA_POSITION.to_vec());
            machine.with_input(10);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn less_than_8_position_mode_true() {
            let mut machine = OpCodeMachine::new(LESS_THAN_8_DATA_POSITION.to_vec());
            machine.with_input(5);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        const LESS_THAN_8_DATA_IMMEDIATE: &'static [i32] = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];

        #[test]
        fn less_than_8_immediate_mode_false() {
            let mut machine = OpCodeMachine::new(LESS_THAN_8_DATA_IMMEDIATE.to_vec());
            machine.with_input(10);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn less_than_8_immediate_mode_true() {
            let mut machine = OpCodeMachine::new(LESS_THAN_8_DATA_IMMEDIATE.to_vec());
            machine.with_input(5);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        const JUMP_DATA_POSITION: &'static [i32] =
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        #[test]
        fn jump_0_position() {
            let mut machine = OpCodeMachine::new(JUMP_DATA_POSITION.to_vec());
            machine.with_input(0);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        const JUMP_DATA_IMMEDIATE: &'static [i32] =
            &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        #[test]
        fn jump_0_immediate() {
            let mut machine = OpCodeMachine::new(JUMP_DATA_IMMEDIATE.to_vec());
            machine.with_input(0);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        const LARGE_EXAMPLE_DATA: &'static [i32] = &[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        #[test]
        fn large_example_input_less_than() {
            let mut machine = OpCodeMachine::new(LARGE_EXAMPLE_DATA.to_vec());
            machine.with_input(5);

            assert_eq!(machine.run(), Some(999));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn large_example_input_equal() {
            let mut machine = OpCodeMachine::new(LARGE_EXAMPLE_DATA.to_vec());
            machine.with_input(8);

            assert_eq!(machine.run(), Some(1000));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn large_example_input_greater_than() {
            let mut machine = OpCodeMachine::new(LARGE_EXAMPLE_DATA.to_vec());
            machine.with_input(10);

            assert_eq!(machine.run(), Some(1001));
            assert_eq!(machine.run(), None);
        }
    }
}
