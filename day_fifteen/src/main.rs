use glam::IVec2;
use winnow::ascii::newline;
use winnow::combinator::{alt, repeat, terminated};
use winnow::error::Result;
use winnow::token::one_of;
use winnow::Parser;

fn main() {
    let mut input = parse_input();

    // Robot starts at middle of map
    let robot_start_position = IVec2 {
        x: (input.warehouse.width / 2 - 1) as i32,
        y: (input.warehouse.height / 2 - 1) as i32,
    };

    println!("{:?}", input.warehouse.contents[1]);
    input.warehouse.push(IVec2 { x: 24, y: 1 }, &BotMove::Right);
    println!("{:?}", input.warehouse.contents[1]);

    println!("{:#?}", robot_start_position);
}

fn parse_input() -> AoCInput {
    let mut input = include_str!("../input.txt");

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
    fn get_next_vec(&self, current_location: IVec2) -> IVec2 {
        match self {
            BotMove::Up => IVec2 {
                x: current_location.x,
                y: current_location.y - 1,
            },
            BotMove::Right => IVec2 {
                x: current_location.x + 1,
                y: current_location.y,
            },
            BotMove::Down => IVec2 {
                x: current_location.x,
                y: current_location.y + 1,
            },
            BotMove::Left => IVec2 {
                x: current_location.x - 1,
                y: current_location.y,
            },
        }
    }

    fn get_prev_vec(&self, current_location: IVec2) -> IVec2 {
        match self {
            BotMove::Up => IVec2 {
                x: current_location.x,
                y: current_location.y + 1,
            },
            BotMove::Right => IVec2 {
                x: current_location.x - 1,
                y: current_location.y,
            },
            BotMove::Down => IVec2 {
                x: current_location.x,
                y: current_location.y - 1,
            },
            BotMove::Left => IVec2 {
                x: current_location.x + 1,
                y: current_location.y,
            },
        }
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

        let new_pusher_location =
            match &self.contents[next_location.y as usize][next_location.x as usize] {
                Content::Box => {
                    let new_pusher_location = self.push(next_location, direction);
                    self.move_item(next_location, new_pusher_location);
                    direction.get_prev_vec(new_pusher_location)
                }
                Content::Empty => self.push(next_location, direction),
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
}
