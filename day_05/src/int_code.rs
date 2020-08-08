pub fn run(mut data: Vec<i32>, input: i32) -> Vec<i32> {
    let mut pointer = 0;
    let mut outputs = Vec::<i32>::new();

    loop {
        let op = data.get(pointer).map(|v| get_op(v)).unwrap();

        match op {
            OpCode::Add(p1_mode, p2_mode) => {
                let p1 = get_value(&data, pointer + 1, p1_mode);
                let p2 = get_value(&data, pointer + 2, p2_mode);
                let p3 = data[pointer + 3];
                data[p3 as usize] = p1 + p2;
                pointer += 4;
            }
            OpCode::Multiply(p1_mode, p2_mode) => {
                let p1 = get_value(&data, pointer + 1, p1_mode);
                let p2 = get_value(&data, pointer + 2, p2_mode);
                let p3 = data[pointer + 3];
                data[p3 as usize] = p1 * p2;
                pointer += 4;
            }
            OpCode::Input() => {
                let address = data[pointer + 1] as usize;
                data[address] = input;
                pointer += 2;
            }
            OpCode::Output(p1_mode) => {
                let value = get_value(&data, pointer + 1, p1_mode).clone();
                outputs.push(value);
                pointer += 2;
            }
            OpCode::JumpIfTrue(p1_mode, p2_mode) => {
                let p1 = get_value(&data, pointer + 1, p1_mode);
                if *p1 == 0 {
                    pointer += 3;
                } else {
                    pointer = *get_value(&data, pointer + 2, p2_mode) as usize;
                }
            }
            OpCode::JumpIfFalse(p1_mode, p2_mode) => {
                let p1 = get_value(&data, pointer + 1, p1_mode);
                if *p1 == 0 {
                    pointer = *get_value(&data, pointer + 2, p2_mode) as usize;
                } else {
                    pointer += 3;
                }
            }
            OpCode::LessThan(p1_mode, p2_mode) => {
                let p1 = get_value(&data, pointer + 1, p1_mode);
                let p2 = get_value(&data, pointer + 2, p2_mode);
                let p3 = *get_value(&data, pointer + 3, ParameterMode::Immediate) as usize;

                data[p3] = if p1 < p2 { 1 } else { 0 };
                pointer += 4;
            }
            OpCode::Equals(p1_mode, p2_mode) => {
                let p1 = get_value(&data, pointer + 1, p1_mode);
                let p2 = get_value(&data, pointer + 2, p2_mode);
                let p3 = *get_value(&data, pointer + 3, ParameterMode::Immediate) as usize;

                data[p3] = if p1 == p2 { 1 } else { 0 };
                pointer += 4;
            }
            OpCode::Halt => break,
        };
    }

    outputs
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
            let data = EQUAL_TO_8_DATA_POSITION.to_vec();
            assert_eq!(run(data, 5), [0]);
        }

        #[test]
        fn example_equal_8_position_mode_true() {
            let data = EQUAL_TO_8_DATA_POSITION.to_vec();
            assert_eq!(run(data, 8), [1]);
        }

        const EQUAL_TO_8_DATA_IMMEDIATE: &'static [i32] = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];

        #[test]
        fn example_equal_8_immediate_mode_false() {
            let data = EQUAL_TO_8_DATA_IMMEDIATE.to_vec();
            assert_eq!(run(data, 5), [0]);
        }

        #[test]
        fn example_equal_8_immediate_mode_true() {
            let data = EQUAL_TO_8_DATA_IMMEDIATE.to_vec();
            assert_eq!(run(data, 8), [1]);
        }

        const LESS_THAN_8_DATA_POSITION: &'static [i32] = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        #[test]
        fn less_than_8_position_mode_false() {
            let data = LESS_THAN_8_DATA_POSITION.to_vec();
            assert_eq!(run(data, 10), [0]);
        }

        #[test]
        fn less_than_8_position_mode_true() {
            let data = LESS_THAN_8_DATA_POSITION.to_vec();
            assert_eq!(run(data, 5), [1]);
        }

        const LESS_THAN_8_DATA_IMMEDIATE: &'static [i32] = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];

        #[test]
        fn less_than_8_immediate_mode_false() {
            let data = LESS_THAN_8_DATA_IMMEDIATE.to_vec();
            assert_eq!(run(data, 10), [0]);
        }

        #[test]
        fn less_than_8_immediate_mode_true() {
            let data = LESS_THAN_8_DATA_IMMEDIATE.to_vec();
            assert_eq!(run(data, 5), [1]);
        }

        const JUMP_DATA_POSITION: &'static [i32] =
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        #[test]
        fn jump_0_position() {
            let data = JUMP_DATA_POSITION.to_vec();
            assert_eq!(run(data, 0), [0]);
        }

        const JUMP_DATA_IMMEDIATE: &'static [i32] =
            &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        #[test]
        fn jump_0_immediate() {
            let data = JUMP_DATA_IMMEDIATE.to_vec();
            assert_eq!(run(data, 0), [0]);
        }

        const LARGE_EXAMPLE_DATA: &'static [i32] = &[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        #[test]
        fn large_example_input_less_than() {
            let data = LARGE_EXAMPLE_DATA.to_vec();

            assert_eq!(run(data, 5), [999]);
        }

        #[test]
        fn large_example_input_equal() {
            let data = LARGE_EXAMPLE_DATA.to_vec();

            assert_eq!(run(data, 8), [1000]);
        }

        #[test]
        fn large_example_input_greater_than() {
            let data = LARGE_EXAMPLE_DATA.to_vec();

            assert_eq!(run(data, 10), [1001]);
        }
    }
}
