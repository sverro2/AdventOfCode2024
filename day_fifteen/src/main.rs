use glam::IVec2;
use winnow::ascii::newline;
use winnow::combinator::{alt, repeat, terminated};
use winnow::error::Result;
use winnow::token::one_of;
use winnow::Parser;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_input(input);

    // Robot starts at middle of map
    let robot_start_position = IVec2 {
        x: (input.warehouse.width / 2 - 1) as i32,
        y: (input.warehouse.height / 2 - 1) as i32,
    };

    part_one(robot_start_position, input.to_owned());
}

fn part_one(robot_start: IVec2, mut input: AoCInput) {
    let mut robot_location = robot_start;

    // Go to each direction (specified in AoC input)
    input.bot_directions.iter().for_each(|direction| {
        robot_location = input.warehouse.push(robot_location, direction);
    });

    println!("Part 1: {}", input.warehouse.calc_gps_all_crates());
}

fn parse_input(mut input: &str) -> AoCInput {
    let warehouse = parse_warehouse(&mut input).unwrap();
    let bot_directions = parse_bot_directions(&mut input).unwrap();

    AoCInput {
        warehouse,
        bot_directions,
    }
}

fn parse_warehouse(input: &mut &str) -> Result<Warehouse> {
    let contents: Vec<Vec<Content>> = repeat(0.., parse_warehouse_row).parse_next(input)?;

    let width = contents[0].len();
    let height = contents.len();

    Ok(Warehouse {
        contents,
        width,
        height,
    })
}

fn parse_warehouse_row(input: &mut &str) -> Result<Vec<Content>> {
    let row_contents = terminated(
        repeat(
            1..,
            alt((
                one_of(('.', '@')).map(|_| Content::Empty),
                '#'.map(|_| Content::Wall),
                'O'.map(|_| Content::Box),
            )),
        ),
        newline,
    )
    .parse_next(input)?;

    Ok(row_contents)
}

fn parse_bot_directions(input: &mut &str) -> Result<Vec<BotMove>> {
    let contents: Vec<Vec<BotMove>> = repeat(.., parse_bot_direction_row).parse_next(input)?;

    Ok(contents.into_iter().flatten().collect())
}

fn parse_bot_direction_row(input: &mut &str) -> Result<Vec<BotMove>> {
    let row_contents = terminated(
        repeat(
            0..,
            alt((
                '^'.map(|_| BotMove::Up),
                '>'.map(|_| BotMove::Right),
                'v'.map(|_| BotMove::Down),
                '<'.map(|_| BotMove::Left),
            )),
        ),
        newline,
    )
    .parse_next(input)?;

    Ok(row_contents)
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
    Box,
    Empty,
    Wall,
}

#[derive(Debug, Clone)]
struct Warehouse {
    contents: Vec<Vec<Content>>,
    width: usize,
    height: usize,
}

impl Warehouse {
    fn push(&mut self, pusher_location: IVec2, direction: &BotMove) -> IVec2 {
        let next_location = direction.get_next_vec(pusher_location);

        // Check what content is at the next location
        let new_pusher_location =
            match &self.contents[next_location.y as usize][next_location.x as usize] {
                // If there's a box, we need to move it to the location closest to the wall that is empty
                Content::Box => {
                    // Recursively push the box
                    let box_destination = self.push(next_location, direction);

                    // Move the box to its new location
                    self.move_item(next_location, box_destination);

                    // Return the position where the pusher should end up (behind the box)
                    direction.get_prev_vec(box_destination)
                }

                // If the next space is empty, continue pushing in that direction
                Content::Empty => next_location,

                // If there's a wall, we can't move, so stay at the current location
                Content::Wall => pusher_location,
            };

        new_pusher_location
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
                    if matches!(self.contents[row_index][column_index], Content::Box) {
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
                    }
                );
            }
            println!();
        }
    }
}
