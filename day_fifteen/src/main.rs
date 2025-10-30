use glam::IVec2;

use crate::{
    parser::{parse_bot_directions, parse_warehouse, parse_warehouse_v2},
    warehouse::Warehouse,
};

mod parser;
mod warehouse;

fn main() {
    let input = include_str!("../input.txt");

    part_one(input);
    part_two(input);
}

fn part_one(input_str: &str) {
    let mut input = parse_input_part_1(input_str.to_owned());
    let mut robot_location = input
        .warehouse
        .get_bot_location()
        .expect("There should be a robot");

    // Go to each direction (specified in AoC input)
    input.bot_directions.iter().for_each(|direction| {
        // Try to push the robot in the given direction
        robot_location = input.warehouse.push(robot_location, direction);
    });

    println!("Part 1: {}", input.warehouse.calc_gps_all_crates());
}

fn part_two(input_str: &str) {
    let mut input = parse_input_part_2(input_str.to_owned());
    let mut robot_location = input
        .warehouse
        .get_bot_location()
        .expect("There should still be a robot");

    // Go to each direction (specified in AoC input)
    input.bot_directions.iter().for_each(|direction| {
        // Try to push the robot in the given direction
        robot_location = input.warehouse.push(robot_location, direction);
    });

    println!("Part 2: {}", input.warehouse.calc_gps_all_crates());
}

fn parse_input_part_1(input: String) -> AoCInput {
    let mut input = input.as_str();
    let warehouse = parse_warehouse(&mut input).unwrap();

    let bot_directions = parse_bot_directions(&mut input).unwrap();

    AoCInput {
        warehouse,
        bot_directions,
    }
}

fn parse_input_part_2(input: String) -> AoCInput {
    let mut input = input.as_str();
    let warehouse = parse_warehouse_v2(&mut input).unwrap();

    let bot_directions = parse_bot_directions(&mut input).unwrap();

    AoCInput {
        warehouse,
        bot_directions,
    }
}

#[derive(Debug, Clone)]
struct AoCInput {
    warehouse: Warehouse,
    bot_directions: Vec<BotMove>,
}

#[derive(Debug, Clone)]
enum BotMove {
    Up,
    Right,
    Down,
    Left,
}

impl BotMove {
    fn direction_vector(&self) -> IVec2 {
        match self {
            BotMove::Up => IVec2 { x: 0, y: -1 },
            BotMove::Right => IVec2 { x: 1, y: 0 },
            BotMove::Down => IVec2 { x: 0, y: 1 },
            BotMove::Left => IVec2 { x: -1, y: 0 },
        }
    }

    fn get_next_vec(&self, current_location: IVec2) -> IVec2 {
        current_location + self.direction_vector()
    }
}

#[derive(Debug, Clone)]
enum Content {
    WideboxLeftPart,
    WideBoxRightPart,
    Box,
    Empty,
    Wall,
    Robot,
}
