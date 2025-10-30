use glam::IVec2;

use crate::parser::{parse_bot_directions, parse_warehouse, parse_warehouse_v2};

mod parser;

fn main() {
    let input = include_str!("../input.txt");

    // // Robot starts at middle of map
    // let robot_start_position = IVec2 {
    //     x: (input.warehouse.width / 2 - 1) as i32,
    //     y: (input.warehouse.height / 2 - 1) as i32,
    // };

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
        // Push just before the robot, so the robot itself also gets pushed around the map
        // New push location is still before the robot, so if we want the location of the robot we need to go one next
        let pusher_location = direction.get_prev_vec(robot_location);
        let next_pusher_location = input.warehouse.push(pusher_location, direction);

        // Now we can see what next robot location is based on push location
        robot_location = direction.get_next_vec(next_pusher_location);
    });

    println!("Part 1: {}", input.warehouse.calc_gps_all_crates());
}

fn part_two(input_str: &str) {
    let mut input = parse_input_part_2(input_str.to_owned());
    let mut robot_location = input
        .warehouse
        .get_bot_location()
        .expect("There should still be a robot");

    input.warehouse.print();
    // Go to each direction (specified in AoC input)
    input.bot_directions.iter().for_each(|direction| {
        // Push just before the robot, so the robot itself also gets pushed around the map
        // New push location is still before the robot, so if we want the location of the robot we need to go one next
        let pusher_location = direction.get_prev_vec(robot_location);
        let next_pusher_location = input.warehouse.push(pusher_location, direction);

        // Now we can see what next robot location is based on push location
        robot_location = direction.get_next_vec(next_pusher_location);
    });

    input.warehouse.print();
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

    fn get_prev_vec(&self, current_location: IVec2) -> IVec2 {
        current_location - self.direction_vector()
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

#[derive(Debug, Clone)]
struct Warehouse {
    contents: Vec<Vec<Content>>,
    width: usize,
    height: usize,
}

impl Warehouse {
    pub fn push(&mut self, pusher_location: IVec2, direction: &BotMove) -> IVec2 {
        if self.can_push(pusher_location, direction) {
            self.push_unchecked(pusher_location, direction)
        } else {
            pusher_location
        }
    }

    fn push_unchecked(&mut self, pusher_location: IVec2, direction: &BotMove) -> IVec2 {
        let next_location = direction.get_next_vec(pusher_location);

        // Check what content is at the next location

        match &self.contents[next_location.y as usize][next_location.x as usize] {
            // In specific cases checks will branch out
            Content::WideboxLeftPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                // Recursively push the pushable
                let pushable_destination = self.push_unchecked(next_location, direction);

                // Move the box to its new location
                self.move_item(next_location, pushable_destination);

                // Move other part of the box as well
                let next_other_part = next_location + IVec2::X;
                let pushable_destination_other_parth =
                    self.push_unchecked(next_other_part, direction);
                self.move_item(next_other_part, pushable_destination_other_parth);

                // Return the position where the pusher should end up (behind the pushable)
                direction.get_prev_vec(pushable_destination)
            }
            Content::WideBoxRightPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                // Recursively push the pushable
                let pushable_destination = self.push_unchecked(next_location, direction);

                // Move the box to its new location
                self.move_item(next_location, pushable_destination);

                // Move other part of the box as well
                let next_other_part = next_location - IVec2::X;
                let pushable_destination_other_parth =
                    self.push_unchecked(next_other_part, direction);
                self.move_item(next_other_part, pushable_destination_other_parth);

                // Return the position where the pusher should end up (behind the pushable)
                direction.get_prev_vec(pushable_destination)
            }
            // Other cases are a bit more simple
            Content::Robot
            | Content::Box
            | Content::WideboxLeftPart
            | Content::WideBoxRightPart => {
                // Recursively push the pushable
                let pushable_destination = self.push_unchecked(next_location, direction);

                // Move the box to its new location
                self.move_item(next_location, pushable_destination);

                // Return the position where the pusher should end up (behind the pushable)
                direction.get_prev_vec(pushable_destination)
            }
            Content::Empty => next_location,
            Content::Wall => pusher_location,
        }
    }

    fn can_push(&mut self, pusher_location: IVec2, direction: &BotMove) -> bool {
        let next_location = direction.get_next_vec(pusher_location);

        match &self.contents[next_location.y as usize][next_location.x as usize] {
            // In specific cases checks will branch out
            Content::WideboxLeftPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.can_push(next_location, direction)
                    && self.can_push(next_location + IVec2::X, direction)
            }
            Content::WideBoxRightPart if matches!(direction, BotMove::Up | BotMove::Down) => {
                self.can_push(next_location, direction)
                    && self.can_push(next_location - IVec2::X, direction)
            }
            // Other cases are a bit more simple
            Content::Robot
            | Content::Box
            | Content::WideboxLeftPart
            | Content::WideBoxRightPart => self.can_push(next_location, direction),
            Content::Empty => true,
            Content::Wall => false,
        }
    }

    fn move_item(&mut self, source_location: IVec2, target_location: IVec2) {
        let source_value = std::mem::replace(
            &mut self.contents[source_location.y as usize][source_location.x as usize],
            Content::Empty,
        );
        self.contents[target_location.y as usize][target_location.x as usize] = source_value;
    }

    fn calc_gps_all_crates(&self) -> usize {
        (0..self.height)
            .flat_map(|row_index| {
                (0..self.width).filter_map(move |column_index| {
                    if matches!(
                        self.contents[row_index][column_index],
                        Content::Box | Content::WideboxLeftPart
                    ) {
                        Some(row_index * 100 + column_index)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }

    fn print(&self) {
        for row in &self.contents {
            for content in row {
                print!(
                    "{}",
                    match content {
                        Content::Box => 'O',
                        Content::Empty => '.',
                        Content::Wall => '#',
                        Content::Robot => '@',
                        Content::WideboxLeftPart => '[',
                        Content::WideBoxRightPart => ']',
                    }
                );
            }
            println!();
        }
    }

    fn get_bot_location(&self) -> Option<IVec2> {
        self.contents
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| {
                row.iter().enumerate().find_map(|(col_index, col)| {
                    if matches!(col, Content::Robot) {
                        Some(IVec2::new(col_index as i32, row_index as i32))
                    } else {
                        None
                    }
                })
            })
    }

    // fn get_robot_start_location
}
