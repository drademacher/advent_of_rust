use regex::Regex;

fn main() {
    println!(
        "First part: {}\nSecond part: {}",
        part_one(parse_inputs(read_input_file())),
        part_two(parse_inputs(read_input_file()))
    );
}

// START: reading input and process it

fn read_input_file() -> Vec<String> {
    let inputs = include_str!("../../resources/day_12.txt")
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<_>>();
    return inputs;
}

fn parse_inputs(inputs: Vec<String>) -> Vec<Action> {
    return inputs.into_iter().map(|s| Action::from(s)).collect();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl Action {
    fn from(raw_value: String) -> Self {
        let capture = Regex::new(r"^(\D)(\d+)$")
            .unwrap()
            .captures_iter(&raw_value)
            .next()
            .expect("invalid input string");

        let movement = capture.get(1).unwrap().as_str().trim();
        let number = capture[2].parse::<usize>().unwrap();

        match movement {
            "N" => Action::North(number),
            "S" => Action::South(number),
            "E" => Action::East(number),
            "W" => Action::West(number),
            "L" => Action::Left(number / 90),
            "R" => Action::Right(number / 90),
            "F" => Action::Forward(number),
            _ => panic!("invalid raw value"),
        }
    }
}

// START: part one

fn part_one(inputs: Vec<Action>) -> isize {
    let mut direction = Direction::East;
    let mut position = Point { x: 0, y: 0 };

    for action in inputs {
        match action {
            Action::North(number) => position.y += number as isize,
            Action::South(number) => position.y -= number as isize,
            Action::East(number) => position.x += number as isize,
            Action::West(number) => position.x -= number as isize,
            Action::Left(number) => direction = direction.rotate_left(number),
            Action::Right(number) => direction = direction.rotate_right(number),
            Action::Forward(number) => match direction {
                Direction::North => position.y += number as isize,
                Direction::South => position.y -= number as isize,
                Direction::East => position.x += number as isize,
                Direction::West => position.x -= number as isize,
            },
        };
    }

    return position.x.abs() + position.y.abs();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(&self, count: usize) -> Self {
        let mut current = self.clone();
        for _ in 0..count {
            current = match current {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            };
        }

        return current;
    }

    fn rotate_left(&self, count: usize) -> Self {
        return self.rotate_right(4 - count);
    }
}

// START: part two

fn part_two(inputs: Vec<Action>) -> isize {
    let mut ship = Point { x: 0, y: 0 };
    let mut waypoint = Point { x: 10, y: 1 };

    for action in inputs {
        match action {
            Action::North(number) => waypoint.y += number as isize,
            Action::South(number) => waypoint.y -= number as isize,
            Action::East(number) => waypoint.x += number as isize,
            Action::West(number) => waypoint.x -= number as isize,
            Action::Left(number) => waypoint = waypoint.rotate_left(number),
            Action::Right(number) => waypoint = waypoint.rotate_right(number),
            Action::Forward(number) => ship = waypoint.multiply(number as isize).add(ship),
        };
    }

    return ship.x.abs() + ship.y.abs();
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn rotate_right(&self, count: usize) -> Self {
        if count == 0 {
            return self.clone();
        }

        return Point {
            x: self.y,
            y: -self.x,
        }
        .rotate_right(count - 1);
    }

    fn rotate_left(&self, count: usize) -> Self {
        return self.rotate_right(4 - count);
    }

    fn multiply(&self, factor: isize) -> Self {
        return Point {
            x: factor * self.x,
            y: factor * self.y,
        };
    }

    fn add(self, other: Point) -> Self {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[test]
fn direction_rotate_right_works_correct() {
    assert_eq!(Direction::East.rotate_right(1), Direction::South);
    assert_eq!(Direction::East.rotate_right(3), Direction::North);
    assert_eq!(Direction::North.rotate_right(0), Direction::North);
    assert_eq!(Direction::North.rotate_right(4), Direction::North);
    assert_eq!(Direction::North.rotate_right(2), Direction::South);
}

#[test]
fn direction_rotate_left_works_correct() {
    assert_eq!(Direction::East.rotate_left(1), Direction::North);
    assert_eq!(Direction::East.rotate_left(3), Direction::South);
    assert_eq!(Direction::North.rotate_left(0), Direction::North);
    assert_eq!(Direction::North.rotate_left(4), Direction::North);
    assert_eq!(Direction::North.rotate_left(2), Direction::South);
}

#[test]
fn point_rotate_right_works_correct() {
    assert_eq!(
        Point { x: 10, y: 4 }.rotate_right(1),
        Point { x: 4, y: -10 }
    );
}

#[test]
fn point_rotate_left_works_correct() {
    assert_eq!(Point { x: 4, y: -10 }.rotate_left(1), Point { x: 10, y: 4 });
}

#[test]
fn point_add_works_correct() {
    assert_eq!(
        Point { x: 4, y: -10 }.add(Point { x: 3, y: 52 }),
        Point { x: 7, y: 42 }
    );
}

#[test]
fn point_multiply_works_correct() {
    assert_eq!(Point { x: 4, y: -10 }.multiply(5), Point { x: 20, y: -50 });
}

#[test]
fn example_part_one() {
    let raw_input = r#"
F10
N3
F7
R90
F11
"#;

    let test_input = raw_input
        .lines()
        .filter(|s| s != &"")
        .map(|s| String::from(s))
        .map(|s| Action::from(s))
        .collect::<Vec<Action>>();

    assert_eq!(part_one(test_input), 25);
}

#[test]
fn example_part_two() {
    let raw_input = r#"
F10
N3
F7
R90
F11
"#;

    let test_input = raw_input
        .lines()
        .filter(|s| s != &"")
        .map(|s| String::from(s))
        .map(|s| Action::from(s))
        .collect::<Vec<Action>>();

    assert_eq!(part_two(test_input), 286);
}
