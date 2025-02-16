use glam::IVec2;
use winnow::ascii::newline;
use winnow::combinator::{alt, repeat, terminated};
use winnow::error::Result;
use winnow::token::one_of;
use winnow::Parser;

fn main() {
    let input = parse_input();

    // Robot starts at middle of map
    let robot_start_position = IVec2 {
        x: (input.warehouse.width / 2 - 1) as i32,
        y: (input.warehouse.height / 2 - 1) as i32,
    };

    println!("{:#?}", input.warehouse);
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

#[derive(Debug)]
struct AoCInput {
    warehouse: Warehouse,
    bot_directions: Vec<BotMove>,
}

#[derive(Debug)]
enum BotMove {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Content {
    Box,
    Empty,
    Wall,
}

#[derive(Debug)]
struct Warehouse {
    contents: Vec<Vec<Content>>,
    width: usize,
    height: usize,
}

struct Bot {}
