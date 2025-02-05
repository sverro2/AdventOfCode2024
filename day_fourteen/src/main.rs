use glam::IVec2;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{separated, separated_pair},
    Parser, Result,
};

fn main() {
    println!("Hello, world!");

    let parsed_bots = parse_restroom_bots();
    println!("{:#?}", parsed_bots);
}

fn parse_restroom_bots() -> Vec<SecurityBotConfig> {
    let mut input = include_str!("../input.txt");

    separated(1.., parse_bot, line_ending)
        .parse_next(&mut input)
        .expect("Unable to parse aoc input")
}

fn parse_bot(input: &mut &str) -> Result<SecurityBotConfig> {
    let (_, location, _, speed) = ("p=", parse_ivec2, " v=", parse_ivec2).parse_next(input)?;

    Ok(SecurityBotConfig { location, speed })
}

fn parse_ivec2(input: &mut &str) -> Result<IVec2> {
    separated_pair(dec_int, ',', dec_int)
        .map(IVec2::from)
        .parse_next(input)
}

#[derive(Debug)]
struct SecurityBotConfig {
    location: IVec2,
    speed: IVec2,
}
