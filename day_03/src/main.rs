use std::cmp;
use std::str::Chars;

fn main() {
    println!("Woo");
}

#[test]
fn example1() {
    let wire1 = Wire::new("R8,U5,L5,D3");
    assert_eq!(wire1.instructions.len(), 4);

    let wire2 = Wire::new("U7,R6,D4,L4");
    assert_eq!(wire2.instructions.len(), 4);

    let cross_points = wire1.get_cross_points(&wire2);

    let closest_point = cross_points.iter().min_by_key(|p| p.location.distance());

    assert_eq!(closest_point.unwrap().location.distance(), 6u32);

    let least_steps_point = cross_points.iter().min_by_key(|p| p.steps);

    assert_eq!(
        least_steps_point,
        Some(&CrossPoint {
            location: Point { x: 6, y: -5 },
            steps: 30
        })
    );
}

#[test]
fn example2() {
    let wire1 = Wire::new("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    assert_eq!(wire1.instructions.len(), 9);

    let wire2 = Wire::new("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(wire2.instructions.len(), 8);

    let cross_points = wire1.get_cross_points(&wire2);

    let closest_point = cross_points.iter().min_by_key(|p| p.location.distance());

    assert_eq!(closest_point.unwrap().location.distance(), 159u32);

    let least_steps_point = cross_points.iter().min_by_key(|p| p.steps);

    assert_eq!(
        least_steps_point,
        Some(&CrossPoint {
            location: Point { x: 158, y: 12 },
            steps: 610
        })
    );
}

#[test]
fn example3() {
    let wire1 = Wire::new("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    assert_eq!(wire1.instructions.len(), 11);

    let wire2 = Wire::new("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(wire2.instructions.len(), 10);

    let cross_points = wire1.get_cross_points(&wire2);

    let closest_point = cross_points.iter().min_by_key(|p| p.location.distance());

    assert_eq!(closest_point.unwrap().location.distance(), 135u32);

    let least_steps_point = cross_points.iter().min_by_key(|p| p.steps);

    assert_eq!(
        least_steps_point,
        Some(&CrossPoint {
            location: Point { x: 107, y: -47 },
            steps: 410
        })
    );
}

#[test]
fn personal_test() {
    let wire1 = Wire::new("R992,U284,L447,D597,R888,D327,R949,U520,R27,U555,L144,D284,R538,U249,R323,U297,R136,U838,L704,D621,R488,U856,R301,U539,L701,U363,R611,D94,L734,D560,L414,U890,R236,D699,L384,D452,R702,D637,L164,U410,R649,U901,L910,D595,R339,D346,R959,U777,R218,D667,R534,D762,R484,D914,L25,U959,R984,D922,R612,U999,L169,D599,L604,D357,L217,D327,L730,D949,L565,D332,L114,D512,R460,D495,L187,D697,R313,U319,L8,D915,L518,D513,R738,U9,R137,U542,L188,U440,R576,D307,R734,U58,R285,D401,R166,U156,L859,U132,L10,U753,L933,U915,R459,D50,R231,D166,L253,U844,R585,D871,L799,U53,R785,U336,R622,D108,R555,D918,L217,D668,L220,U738,L997,D998,R964,D456,L54,U930,R985,D244,L613,D116,L994,D20,R949,D245,L704,D564,L210,D13,R998,U951,L482,U579,L793,U680,L285,U770,L975,D54,R79,U613,L907,U467,L256,D783,R883,U810,R409,D508,L898,D286,L40,U741,L759,D549,R210,U411,R638,D643,L784,U538,L739,U771,L773,U491,L303,D425,L891,U182,R412,U951,L381,U501,R482,D625,R870,D320,L464,U555,R566,D781,L540,D754,L211,U73,L321,D869,R994,D177,R496,U383,R911,U819,L651,D774,L591,U666,L883,U767,R232,U822,L499,U44,L45,U873,L98,D487,L47,U803,R855,U256,R567,D88,R138,D678,L37,U38,R783,U569,L646,D261,L597,U275,L527,U48,R433,D324,L631,D160,L145,D128,R894,U223,R664,U510,R756,D700,R297,D361,R837,U996,L769,U813,L477,U420,L172,U482,R891,D379,L329,U55,R284,U155,L816,U659,L671,U996,R997,U252,R514,D718,L661,D625,R910,D960,L39,U610,R853,U859,R174,U215,L603,U745,L587,D736,R365,U78,R306,U158,L813,U885,R558,U631,L110,D232,L519,D366,R909,D10,R294");
    assert_eq!(wire1.instructions.len(), 301);

    let wire2 = Wire::new("L1001,D833,L855,D123,R36,U295,L319,D700,L164,U576,L68,D757,R192,D738,L640,D660,R940,D778,R888,U772,R771,U900,L188,D464,L572,U184,R889,D991,L961,U751,R560,D490,L887,D748,R37,U910,L424,D401,L385,U415,L929,U193,R710,D855,L596,D323,L966,D505,L422,D139,L108,D135,R737,U176,R538,D173,R21,D951,R949,D61,L343,U704,R127,U468,L240,D834,L858,D127,R328,D863,R329,U477,R131,U864,R997,D38,R418,U611,R28,U705,R148,D414,R786,U264,L785,D650,R201,D250,R528,D910,R670,U309,L658,U190,R704,U21,R288,D7,R930,U62,R782,U621,R328,D725,R305,U700,R494,D137,R969,U142,L867,U577,R300,U162,L13,D698,R333,U865,R941,U796,L60,U902,L784,U832,R78,D578,R196,D390,R728,D922,R858,D994,L457,U547,R238,D345,R329,D498,R873,D212,R501,U474,L657,U910,L335,U133,R213,U417,R698,U829,L2,U704,L273,D83,R231,D247,R675,D23,L692,D472,L325,D659,L408,U746,L715,U395,L596,U296,R52,D849,L713,U815,R684,D551,L319,U768,R176,D182,R557,U731,R314,D543,L9,D256,R38,D809,L567,D332,R375,D572,R81,D479,L71,U968,L831,D247,R989,U390,R463,D576,R740,D539,R488,U367,L596,U375,L763,D824,R70,U448,R979,D977,L744,D379,R488,D671,L516,D334,L542,U517,L488,D390,L713,D932,L28,U924,L448,D229,L488,D501,R19,D910,L979,D411,R711,D824,L973,U291,R794,D485,R208,U370,R655,U450,L40,D804,L374,D671,R962,D829,L209,U111,L84,D876,L832,D747,L733,D560,L702,D972,R188,U817,L111,U26,L492,U485,L71,D59,L269,D870,L152,U539,R65,D918,L932,D260,L485,U77,L699,U254,R924,U643,L264,U96,R395,D917,R360,U354,R101,D682,R854,U450,L376,D378,R872,D311,L881,U630,R77,D766,R672");
    assert_eq!(wire2.instructions.len(), 301);

    let cross_points = wire1.get_cross_points(&wire2);

    let closest_point = cross_points.iter().min_by_key(|p| p.location.distance());

    assert_eq!(closest_point.unwrap().location.distance(), 5357u32);

    let least_steps_point = cross_points.iter().min_by_key(|p| p.steps);

    assert_eq!(
        least_steps_point,
        Some(&CrossPoint {
            location: Point { x: 4240, y: 1272 },
            steps: 101956
        })
    );
}

pub struct Wire {
    instructions: Vec<Instruction>,
}

impl Wire {
    fn new(instructions_raw: &str) -> Wire {
        let instructions: Vec<Instruction> = instructions_raw
            .split(',')
            .map(|instruction| parse_instruction(instruction.chars()))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        let mut points: Vec<Point> = vec![Point { x: 0, y: 0 }];

        for instruction in instructions.iter() {
            let last = points.last().unwrap();
            let mut x = last.x;
            let mut y = last.y;

            match instruction {
                Instruction::Up(v) => x -= *v as i32,
                Instruction::Down(v) => x += *v as i32,
                Instruction::Left(v) => y -= *v as i32,
                Instruction::Right(v) => y += *v as i32,
            }

            points.push(Point { x, y });
        }

        Wire { instructions }
    }

    fn get_cross_points(&self, other: &Wire) -> Vec<CrossPoint> {
        let mut self_point = Point { x: 0, y: 0 };
        let mut cross_points = Vec::new();
        let mut self_steps = 0u32;

        for self_instruction in self.instructions.iter() {
            let (self_axis, self_amount) = devolve_instruction(self_instruction);
            let mut other_point = Point { x: 0, y: 0 };
            let mut other_steps = 0u32;

            for other_instruction in other.instructions.iter() {
                let (other_axis, other_amount) = devolve_instruction(other_instruction);

                if other_axis != self_axis {
                    // Check to see if they cross
                    let maybe_cross_point = if self_axis == Axis::Horizontal {
                        is_crossing(&self_point, self_amount, &other_point, other_amount)
                    } else {
                        is_crossing(&other_point, other_amount, &self_point, self_amount)
                    };

                    if let Some(cross_point) = maybe_cross_point {
                        let cross_self_steps = get_steps(&self_point, &cross_point);
                        let cross_other_steps = get_steps(&other_point, &cross_point);

                        cross_points.push(CrossPoint {
                            location: cross_point,
                            steps: self_steps + other_steps + cross_self_steps + cross_other_steps,
                        });
                    }
                }

                other_steps += other_amount.abs() as u32;

                match other_axis {
                    Axis::Horizontal => other_point.x += other_amount,
                    Axis::Vertical => other_point.y += other_amount,
                }
            }

            self_steps += self_amount.abs() as u32;

            match self_axis {
                Axis::Horizontal => self_point.x += self_amount,
                Axis::Vertical => self_point.y += self_amount,
            }
        }

        cross_points
    }
}

fn get_steps(a: &Point, b: &Point) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

fn is_crossing(
    horizontal_start: &Point,
    horizontal_delta: i32,
    vertical_start: &Point,
    vertical_delta: i32,
) -> Option<Point> {
    let x_delta = horizontal_start.x + horizontal_delta;
    let left = cmp::min(horizontal_start.x, x_delta);
    let right = cmp::max(horizontal_start.x, x_delta);

    let y_delta = vertical_start.y + vertical_delta;
    let top = cmp::min(vertical_start.y, y_delta);
    let bottom = cmp::max(vertical_start.y, y_delta);

    if vertical_start.x > left
        && vertical_start.x < right
        && horizontal_start.y > top
        && horizontal_start.y < bottom
    {
        return Some(Point {
            x: vertical_start.x,
            y: horizontal_start.y,
        });
    }

    None
}

fn devolve_instruction(instruction: &Instruction) -> (Axis, i32) {
    match instruction {
        Instruction::Up(v) => (Axis::Vertical, -(*v as i32)),
        Instruction::Down(v) => (Axis::Vertical, *v as i32),
        Instruction::Left(v) => (Axis::Horizontal, -(*v as i32)),
        Instruction::Right(v) => (Axis::Horizontal, *v as i32),
    }
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

#[derive(Debug)]
pub struct CrossPoint {
    location: Point,
    steps: u32,
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::cmp::PartialEq for CrossPoint {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps && self.location == other.location
    }
}

#[derive(PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

pub enum Instruction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

fn parse_instruction(mut instruction: Chars) -> Result<Instruction, &'static str> {
    // Note: doing nth(0) moves the iterator, so there's no need to skip plater
    match instruction.nth(0).unwrap_or('X') {
        'U' => to_instruction(instruction, |v| Instruction::Up(v)),
        'D' => to_instruction(instruction, |v| Instruction::Down(v)),
        'L' => to_instruction(instruction, |v| Instruction::Left(v)),
        'R' => to_instruction(instruction, |v| Instruction::Right(v)),
        _ => Err("Unknown"),
    }
}

fn to_instruction<F>(instruction: Chars, converter: F) -> Result<Instruction, &'static str>
where
    F: FnOnce(u32) -> Instruction,
{
    instruction
        .collect::<String>()
        .parse::<u32>()
        .map(converter)
        .map_err(|_| "NaN")
}
