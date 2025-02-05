use std::ops::Add;
use winnow::{
    ascii::newline,
    combinator::{dispatch, empty, fail, repeat, terminated},
    error::{StrContext, StrContextValue},
    token::any,
};
use winnow::{Parser, Result};

fn main() {
    parse_input();
}

fn parse_input() {
    let input = &include_str!("../input.txt");
    let maze = input.parse::<Maze>().unwrap();

    part_1(maze.to_owned());
    part_2(maze);
}

fn part_1(mut maze: Maze) {
    let mut guard_direction = Direction::North;
    let mut current_guard_location = locate_guard(&maze);

    loop {
        let next_chamber_index = &current_guard_location + guard_direction.heading();
        if let Some(next_chamber) = maze.get_at(&next_chamber_index) {
            if matches!(next_chamber, Chamber::Obstruction) {
                guard_direction = guard_direction.rotate_clockwise();
                continue;
            }
            maze.mark_guard_path(&next_chamber_index, guard_direction.to_owned());
            current_guard_location = next_chamber_index;
        } else {
            break;
        }
    }

    println!("Guard locations: {}", count_guard_locations(&maze));
}

fn part_2(maze: Maze) {
    let mut locations_to_check = vec![];

    // Find out which other obstacles can be placed in chambers of the maze
    for y in 0..maze.rows.len() {
        for x in 0..maze.rows[y].len() {
            if !matches!(&maze.rows[y][x], Chamber::Obstruction | Chamber::Guard(_)) {
                locations_to_check.push(Point(x as isize, y as isize));
            }
        }
    }

    // for each location, try out if it contains a loop
    let loop_count = locations_to_check
        .iter()
        .filter(|location| {
            let mut maze_to_check = maze.clone();
            maze_to_check.mark_obstructed(location);
            detect_loop_in_maze(&mut maze_to_check)
        })
        .count();

    println!("Loop count: {}", loop_count);
}

fn detect_loop_in_maze(maze: &mut Maze) -> bool {
    let mut guard_direction = Direction::North;
    let mut current_guard_location = locate_guard(maze);

    let mut loop_detected = false;

    loop {
        let next_chamber_index = &current_guard_location + guard_direction.heading();
        if let Some(next_chamber) = maze.get_at(&next_chamber_index) {
            match next_chamber {
                Chamber::Obstruction => {
                    guard_direction = guard_direction.rotate_clockwise();
                    continue;
                }
                Chamber::Guard(next_direction) if *next_direction == guard_direction => {
                    loop_detected = true;
                    break;
                }
                _ => (),
            }
            maze.mark_guard_path(&next_chamber_index, guard_direction.to_owned());
            current_guard_location = next_chamber_index;
        } else {
            break;
        }
    }

    loop_detected
}

fn parse_maze(input: &mut &str) -> Result<Vec<Vec<Chamber>>> {
    repeat(1.., parse_maze_row).parse_next(input)
}

fn parse_maze_row(input: &mut &str) -> Result<Vec<Chamber>> {
    terminated(repeat(1.., parse_chamber), newline).parse_next(input)
}

fn parse_chamber(input: &mut &str) -> Result<Chamber> {
    dispatch!(any;
        '.' =>  empty.value(Chamber::Empty),
        '#' => empty.value(Chamber::Obstruction),
        '^' => empty.value(Chamber::Guard(Direction::North)),
        _ => fail.context(StrContext::Label("chamber"))
            .context(StrContext::Expected(StrContextValue::CharLiteral('^')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('#')))
            .context(StrContext::Expected(StrContextValue::CharLiteral('.')))
    )
    .parse_next(input)
}

fn locate_guard(maze: &Maze) -> Point {
    for y in 0..maze.rows.len() {
        for x in 0..maze.rows[y].len() {
            if let Chamber::Guard(_) = &maze.rows[y][x] {
                return Point(x as isize, y as isize);
            }
        }
    }
    unreachable!()
}

fn count_guard_locations(maze: &Maze) -> usize {
    let mut guard_locations = 0;
    for y in 0..maze.rows.len() {
        for x in 0..maze.rows[y].len() {
            if let Chamber::Guard(_) = &maze.rows[y][x] {
                guard_locations += 1;
            }
        }
    }
    guard_locations
}

#[derive(Clone, Debug)]
enum Chamber {
    Empty,
    Obstruction,
    Guard(Direction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn heading(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone)]
struct Point(isize, isize);

impl Add<(isize, isize)> for &Point {
    type Output = Point;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
struct Maze {
    rows: Vec<Vec<Chamber>>,
}

impl Maze {
    fn get_at(&self, point: &Point) -> Option<&Chamber> {
        let x: usize = point.0.try_into().ok()?;
        let y: usize = point.1.try_into().ok()?;
        self.rows.get(y).and_then(|row| row.get(x))
    }

    fn mark_guard_path(&mut self, point: &Point, direction: Direction) {
        self.rows[point.1 as usize][point.0 as usize] = Chamber::Guard(direction);
    }

    fn mark_obstructed(&mut self, point: &Point) {
        self.rows[point.1 as usize][point.0 as usize] = Chamber::Obstruction;
    }
}

impl std::str::FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_maze
            .parse(input)
            .map(|rows| Maze { rows })
            .map_err(|e| anyhow::format_err!("{e}"))
    }
}
