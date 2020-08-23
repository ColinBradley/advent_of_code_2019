pub struct OpCodeMachine<F: Fn() -> i64> {
    data: Vec<i64>,
    input_provider: F,
    pointer: usize,
    relative_base: isize,
    pub is_complete: bool,
}

impl<F: Fn() -> i64> OpCodeMachine<F> {
    pub fn new(data: Vec<i64>, input_provider: F) -> OpCodeMachine<F> {
        OpCodeMachine {
            data,
            input_provider,
            pointer: 0,
            relative_base: 0,
            is_complete: false,
        }
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

                    let value = (self.input_provider)();

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
