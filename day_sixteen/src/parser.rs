use std::collections::HashSet;

use crate::{AoCInput, Position};

pub fn parse_input(input: &str) -> AoCInput {
    let mut start = None;
    let mut finish = None;
    let mut walls = HashSet::new();

    for (line_index, line) in input.lines().enumerate() {
        for (column_index, char) in line.chars().enumerate() {
            match char {
                'S' => start = Some(Position::new(column_index, line_index)),
                'E' => finish = Some(Position::new(column_index, line_index)),
                '#' => {
                    walls.insert(Position::new(column_index, line_index));
                }
                _ => {}
            }
        }
    }

    let (Some(start), Some(finish)) = (start, finish) else {
        panic!("AOC input doesn't seem correct. We miss S(tart) or E(nd) location!");
    };

    AoCInput::new(walls, start, finish)
}
