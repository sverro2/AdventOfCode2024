use crate::{BotMove, Content, Warehouse};

use winnow::ascii::newline;
use winnow::combinator::{alt, repeat, terminated};
use winnow::error::Result;
use winnow::Parser;

pub fn parse_warehouse(input: &mut &str) -> Result<Warehouse> {
    let contents: Vec<Vec<Content>> = repeat(0.., parse_warehouse_row).parse_next(input)?;

    let width = contents[0].len();
    let height = contents.len();

    Ok(Warehouse::new(contents, width, height))
}

fn parse_warehouse_row(input: &mut &str) -> Result<Vec<Content>> {
    let row_contents = terminated(
        repeat(
            1..,
            alt((
                '@'.map(|_| Content::Robot),
                '.'.map(|_| Content::Empty),
                '#'.map(|_| Content::Wall),
                'O'.map(|_| Content::Box),
            )),
        ),
        newline,
    )
    .parse_next(input)?;

    Ok(row_contents)
}

pub fn parse_warehouse_v2(input: &mut &str) -> Result<Warehouse> {
    let contents: Vec<Vec<Content>> = repeat(0.., parse_warehouse_row_v2).parse_next(input)?;

    let width = contents[0].len();
    let height = contents.len();

    Ok(Warehouse::new(contents, width, height))
}

fn parse_warehouse_row_v2(input: &mut &str) -> Result<Vec<Content>> {
    let row_contents = terminated(
        repeat(
            1..,
            alt((
                '@'.map(|_| [Content::Robot, Content::Empty]),
                '.'.map(|_| [Content::Empty, Content::Empty]),
                '#'.map(|_| [Content::Wall, Content::Wall]),
                'O'.map(|_| [Content::WideboxLeftPart, Content::WideBoxRightPart]),
            )),
        )
        .map(|i: Vec<_>| i.into_iter().flatten().collect::<Vec<_>>()),
        newline,
    )
    .parse_next(input)?;

    Ok(row_contents)
}

pub fn parse_bot_directions(input: &mut &str) -> Result<Vec<BotMove>> {
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
