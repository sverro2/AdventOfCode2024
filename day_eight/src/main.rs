use winnow::{
    ascii::line_ending,
    combinator::{alt, empty, repeat, separated},
    dispatch,
    token::{any, one_of},
    PResult, Parser,
};

#[derive(Debug)]
enum RoofTile {
    Empty,
    Antenna(char),
}

fn main() {
    let input = parse_input();
}

fn parse_input() {
    let mut input = include_str!("../input.txt");
    let parsed = parse_roof(&mut input).unwrap();
    println!("{parsed:?}");
    println!("{input}");
}

fn parse_roof(input: &mut &str) -> PResult<Vec<Vec<RoofTile>>> {
    separated(1.., parse_roof_row, line_ending).parse_next(input)
}

fn parse_roof_row(input: &mut &str) -> PResult<Vec<RoofTile>> {
    repeat(
        1..,
        alt((
            '.'.map(|_| RoofTile::Empty),
            one_of(('a'..='z', 'A'..='Z', '0'..='9')).map(|c| RoofTile::Antenna(c)),
        )),
    )
    .parse_next(input)
}
