pub fn run(mut data: Vec<i32>, input: i32) -> Vec<i32> {
    let mut pointer = 0;
    let mut outputs = Vec::<i32>::new();

    loop {
        let op = match data.get(pointer) {
            Some(v) => get_op(v),
            None => break,
        };

        match op {
            OpCode::Add(p1_mode, p2_mode, _) => {
                let p1 = get_value(&data, pointer + 1, p1_mode).unwrap();
                let p2 = get_value(&data, pointer + 2, p2_mode).unwrap();
                let p3 = data[pointer + 3];
                data[p3 as usize] = p1 + p2;
                pointer += 4;
            }
            OpCode::Multiply(p1_mode, p2_mode, _) => {
                let p1 = get_value(&data, pointer + 1, p1_mode).unwrap();
                let p2 = get_value(&data, pointer + 2, p2_mode).unwrap();
                let p3 = data[pointer + 3];
                data[p3 as usize] = p1 * p2;
                pointer += 4;
            }
            OpCode::Input(_) => {
                let address = data[pointer + 1] as usize;
                data[address] = input;
                pointer += 2;
            }
            OpCode::Output(p1_mode) => {
                outputs.push(get_value(&data, pointer + 1, p1_mode).unwrap().clone());
                pointer += 2;
            }
            OpCode::Halt => break,
        }
    }

    outputs
}

fn get_value<'a>(data: &'a Vec<i32>, location: usize, mode: ParameterMode) -> Option<&'a i32> {
    match mode {
        ParameterMode::Immediate => data.get(location),
        ParameterMode::Position => data.get(*data.get(location)? as usize),
    }
}

fn get_op(value: &i32) -> OpCode {
    let ((p1, p2, p3), operation) = parse_operation(value);

    match operation {
        1 => OpCode::Add(p1.into(), p2.into(), p3.into()),
        2 => OpCode::Multiply(p1.into(), p2.into(), p3.into()),
        3 => OpCode::Input(p1.into()),
        4 => OpCode::Output(p1.into()),
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
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
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
}
