use itertools::Itertools;
use winnow::ascii::line_ending;
use winnow::combinator::repeat;
use winnow::combinator::separated;
use winnow::token::any;
use winnow::Parser;
use winnow::Result;

#[derive(Debug)]
struct Map {
    coordinate_data: Vec<Vec<u8>>,
}

impl Map {
    fn coordinate_from(&self, x: i32, y: i32) -> Option<(Coordinate, u8)> {
        if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
            self.get(x, y).map(|value| (Coordinate::new(x, y), value))
        } else {
            None
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.coordinate_data
            .get(y)
            .and_then(|row| row.get(x).copied())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct Heading {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn main() {
    let map = parse_input();
    let possible_headings = [
        Heading { x: 0, y: 1 },
        Heading { x: 0, y: -1 },
        Heading { x: 1, y: 0 },
        Heading { x: -1, y: 0 },
    ];

    let potential_trail_start_coordinates: Vec<_> = map
        .coordinate_data
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &value)| value == 0)
                .map(move |(col_index, _)| Coordinate::new(col_index, row_index))
        })
        .collect();

    let trail_ending_coordinates: Vec<_> = potential_trail_start_coordinates
        .iter()
        .map(|start_coordinate| {
            // Depth first search, counting destinations
            let mut trail_end_coordinates = vec![];
            let mut next_coordinates_to_check = vec![(*start_coordinate, 0)];

            while let Some((coordinate, current_value)) = next_coordinates_to_check.pop() {
                possible_headings
                    .iter()
                    .flat_map(|heading| {
                        map.coordinate_from(
                            coordinate.x as i32 + heading.x,
                            coordinate.y as i32 + heading.y,
                        )
                    })
                    .filter(|(next_coordinate, next_value)| {
                        let valid_next_value = *next_value == current_value + 1;

                        // if 9, we reached the end of the trail.
                        // register this, but we don't need to check for a continuation of this trail
                        if *next_value == 9 && valid_next_value {
                            trail_end_coordinates.push(*next_coordinate);
                            false
                        } else {
                            valid_next_value
                        }
                    })
                    .for_each(|next| next_coordinates_to_check.push(next));
            }

            trail_end_coordinates
        })
        .collect();

    // First we filter out all endings that have routes connecting to it multiple times
    println!(
        "Part 1, unique start-to-end: {}",
        trail_ending_coordinates
            .iter()
            .flat_map(|f| f.iter().unique())
            .count()
    );

    // Then... we don't need to anymore ;)
    println!(
        "Part 2, all possible trails: {}",
        trail_ending_coordinates.iter().flatten().count()
    );
}

fn parse_input() -> Map {
    let mut input = include_str!("../input.txt");

    Map {
        coordinate_data: parse_map(&mut input).unwrap(),
    }
}

fn parse_map(input: &mut &str) -> Result<Vec<Vec<u8>>> {
    separated(1.., parse_map_row, line_ending).parse_next(input)
}

fn parse_map_row(input: &mut &str) -> Result<Vec<u8>> {
    repeat(
        1..,
        any.verify(|c: &char| c.is_ascii_digit())
            .map(|c: char| c.to_digit(10).unwrap() as u8),
    )
    .parse_next(input)
}
