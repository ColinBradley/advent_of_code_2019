pub struct OpCodeMachine {
    data: Vec<i64>,
    inputs: Vec<i64>,
    pointer: usize,
    relative_base: isize,
    pub is_complete: bool,
}

impl OpCodeMachine {
    pub fn new(data: Vec<i64>) -> OpCodeMachine {
        OpCodeMachine {
            data,
            inputs: Vec::new(),
            pointer: 0,
            relative_base: 0,
            is_complete: false,
        }
    }

    #[cfg(test)]
    pub fn with_input(mut self, input: i64) -> OpCodeMachine {
        self.inputs.push(input);
        self
    }

    pub fn input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn run(&mut self) -> Option<i64> {
        if self.is_complete {
            return None;
        }

        loop {
            let op = self.data.get(self.pointer).map(|v| get_op(v)).unwrap();

            match op {
                OpCode::Add(p1_mode, p2_mode, p3_mode) => {
                    let p1 = self.get_data(self.pointer + 1, p1_mode);
                    let p2 = self.get_data(self.pointer + 2, p2_mode);
                    let p3 = self.get_index(self.pointer + 3, p3_mode);

                    let value = p1 + p2;

                    self.set_value(p3, value);
                    self.pointer += 4;
                }
                OpCode::Multiply(p1_mode, p2_mode, p3_mode) => {
                    let p1 = self.get_data(self.pointer + 1, p1_mode);
                    let p2 = self.get_data(self.pointer + 2, p2_mode);
                    let p3 = self.get_index(self.pointer + 3, p3_mode);

                    let value = p1 * p2;

                    self.set_value(p3 as usize, value);
                    self.pointer += 4;
                }
                OpCode::Input(p1_mode) => {
                    let p1 = self.get_index(self.pointer + 1, p1_mode);

                    let value = self.inputs.remove(0);

                    self.set_value(p1, value);
                    self.pointer += 2;
                }
                OpCode::Output(p1_mode) => {
                    let p1 = *self.get_data(self.pointer + 1, p1_mode);

                    self.pointer += 2;
                    return Some(p1);
                }
                OpCode::JumpIfTrue(p1_mode, p2_mode) => {
                    let p1 = self.get_data(self.pointer + 1, p1_mode);

                    if *p1 == 0 {
                        self.pointer += 3;
                    } else {
                        self.pointer = *self.get_data(self.pointer + 2, p2_mode) as usize;
                    }
                }
                OpCode::JumpIfFalse(p1_mode, p2_mode) => {
                    let p1 = self.get_data(self.pointer + 1, p1_mode);

                    if *p1 == 0 {
                        self.pointer = *self.get_data(self.pointer + 2, p2_mode) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                OpCode::LessThan(p1_mode, p2_mode, p3_mode) => {
                    let p1 = self.get_data(self.pointer + 1, p1_mode);
                    let p2 = self.get_data(self.pointer + 2, p2_mode);
                    let p3 = self.get_index(self.pointer + 3, p3_mode);

                    let value = if p1 < p2 { 1 } else { 0 };

                    self.set_value(p3, value);
                    self.pointer += 4;
                }
                OpCode::Equals(p1_mode, p2_mode, p3_mode) => {
                    let p1 = self.get_data(self.pointer + 1, p1_mode);
                    let p2 = self.get_data(self.pointer + 2, p2_mode);
                    let p3 = self.get_index(self.pointer + 3, p3_mode);

                    let value = if p1 == p2 { 1 } else { 0 };

                    self.set_value(p3, value);
                    self.pointer += 4;
                }
                OpCode::AdjustRelativeBase(p1_mode) => {
                    let p1 = self.get_data(self.pointer + 1, p1_mode);

                    self.relative_base += *p1 as isize;
                    self.pointer += 2;
                }
                OpCode::Halt => {
                    self.is_complete = true;
                    return None;
                }
            };
        }
    }

    fn set_value(&mut self, location: usize, value: i64) {
        if location >= self.data.len() {
            self.data.extend((self.data.len()..=location).map(|_| 0i64));
        }
        self.data[location] = value;
    }

    fn get_data(&self, location: usize, mode: ParameterMode) -> &i64 {
        self.get_value_direct(self.get_index(location, mode))
    }

    fn get_index(&self, location: usize, mode: ParameterMode) -> usize {
        match mode {
            ParameterMode::Position => *self.get_value_direct(location) as usize,
            ParameterMode::Immediate => location,
            ParameterMode::Relative => {
                let read_value = *self.get_value_direct(location) as isize;
                (read_value + self.relative_base) as usize
            }
        }
    }

    fn get_value_direct(&self, location: usize) -> &i64 {
        if location >= self.data.len() {
            return &0i64;
        }

        self.data.get(location).unwrap()
    }
}

fn get_op(value: &i64) -> OpCode {
    let ((p1, p2, p3), operation) = parse_operation(value);

    match operation {
        1 => OpCode::Add(p1.into(), p2.into(), p3.into()),
        2 => OpCode::Multiply(p1.into(), p2.into(), p3.into()),
        3 => OpCode::Input(p1.into()),
        4 => OpCode::Output(p1.into()),
        5 => OpCode::JumpIfTrue(p1.into(), p2.into()),
        6 => OpCode::JumpIfFalse(p1.into(), p2.into()),
        7 => OpCode::LessThan(p1.into(), p2.into(), p3.into()),
        8 => OpCode::Equals(p1.into(), p2.into(), p3.into()),
        9 => OpCode::AdjustRelativeBase(p1.into()),
        99 => OpCode::Halt,
        _ => panic!("unknown code"),
    }
}

fn parse_operation(value: &i64) -> ((i64, i64, i64), i64) {
    let operation = value % 100;

    let p1 = value / 100 % 10;
    let p2 = value / 1_000 % 10;
    let p3 = value / 10_000 % 10;

    ((p1, p2, p3), operation)
}

enum OpCode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    AdjustRelativeBase(ParameterMode),
    Halt,
}

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> ParameterMode {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
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

        const EQUAL_TO_8_DATA_POSITION: &'static [i64] = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        #[test]
        fn example_equal_8_position_mode_false() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_POSITION.to_vec()).with_input(5);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn example_equal_8_position_mode_true() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_POSITION.to_vec()).with_input(8);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        const EQUAL_TO_8_DATA_IMMEDIATE: &'static [i64] = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];

        #[test]
        fn example_equal_8_immediate_mode_false() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_IMMEDIATE.to_vec()).with_input(5);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn example_equal_8_immediate_mode_true() {
            let mut machine = OpCodeMachine::new(EQUAL_TO_8_DATA_IMMEDIATE.to_vec()).with_input(8);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        const LESS_THAN_8_DATA_POSITION: &'static [i64] = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        #[test]
        fn less_than_8_position_mode_false() {
            let mut machine = OpCodeMachine::new(LESS_THAN_8_DATA_POSITION.to_vec()).with_input(10);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn less_than_8_position_mode_true() {
            let mut machine = OpCodeMachine::new(LESS_THAN_8_DATA_POSITION.to_vec()).with_input(5);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        const LESS_THAN_8_DATA_IMMEDIATE: &'static [i64] = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];

        #[test]
        fn less_than_8_immediate_mode_false() {
            let mut machine =
                OpCodeMachine::new(LESS_THAN_8_DATA_IMMEDIATE.to_vec()).with_input(10);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn less_than_8_immediate_mode_true() {
            let mut machine = OpCodeMachine::new(LESS_THAN_8_DATA_IMMEDIATE.to_vec()).with_input(5);

            assert_eq!(machine.run(), Some(1));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn jump_0_position() {
            let mut machine = OpCodeMachine::new(vec![
                3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
            ])
            .with_input(0);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn jump_0_immediate() {
            let mut machine =
                OpCodeMachine::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1])
                    .with_input(0);

            assert_eq!(machine.run(), Some(0));
            assert_eq!(machine.run(), None);
        }

        const LARGE_EXAMPLE_DATA: &'static [i64] = &[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        #[test]
        fn large_example_input_less_than() {
            let mut machine = OpCodeMachine::new(LARGE_EXAMPLE_DATA.to_vec()).with_input(5);

            assert_eq!(machine.run(), Some(999));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn large_example_input_equal() {
            let mut machine = OpCodeMachine::new(LARGE_EXAMPLE_DATA.to_vec()).with_input(8);

            assert_eq!(machine.run(), Some(1000));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn large_example_input_greater_than() {
            let mut machine = OpCodeMachine::new(LARGE_EXAMPLE_DATA.to_vec()).with_input(10);

            assert_eq!(machine.run(), Some(1001));
            assert_eq!(machine.run(), None);
        }

        #[test]
        fn large_value_support_example() {
            let mut machine = OpCodeMachine::new(vec![104, 1125899906842624, 99]);
            assert_eq!(machine.run(), Some(1125899906842624));
        }

        #[test]
        fn quine_example() {
            let source = vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            let mut machine = OpCodeMachine::new(source.clone());

            for value in source.iter() {
                assert_eq!(machine.run(), Some(*value));
            }
        }

        #[test]
        fn sixteen_digit_output_example() {
            let mut machine = OpCodeMachine::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
            assert_eq!(machine.run(), Some(1219070632396864));
        }

        #[test]
        fn extra_1_example() {
            let mut machine = OpCodeMachine::new(vec![109, -1, 4, 1, 99]);
            assert_eq!(machine.run(), Some(-1));
        }

        #[test]
        fn extra_2_example() {
            let mut machine = OpCodeMachine::new(vec![109, -1, 104, 1, 99]);
            assert_eq!(machine.run(), Some(1));
        }

        #[test]
        fn extra_3_example() {
            let mut machine = OpCodeMachine::new(vec![109, -1, 204, 1, 99]);
            assert_eq!(machine.run(), Some(109));
        }

        #[test]
        fn extra_4_example() {
            let mut machine = OpCodeMachine::new(vec![109, 1, 9, 2, 204, -6, 99]);
            assert_eq!(machine.run(), Some(204));
        }

        #[test]
        fn extra_5_example() {
            let mut machine = OpCodeMachine::new(vec![109, 1, 109, 9, 204, -6, 99]);
            assert_eq!(machine.run(), Some(204));
        }

        #[test]
        fn extra_6_example() {
            let mut machine = OpCodeMachine::new(vec![109, 1, 209, -1, 204, -106, 99]);
            assert_eq!(machine.run(), Some(204));
        }

        #[test]
        fn extra_7_example() {
            let mut machine = OpCodeMachine::new(vec![109, 1, 3, 3, 204, 2, 99]).with_input(42);
            assert_eq!(machine.run(), Some(42));
        }

        #[test]
        fn extra_8_example() {
            let mut machine = OpCodeMachine::new(vec![109, 1, 203, 2, 204, 2, 99]).with_input(42);
            assert_eq!(machine.run(), Some(42));
        }
    }
}
