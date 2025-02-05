use glam::I8Vec2;
use itertools::Itertools;
use winnow::{
    ascii::line_ending,
    combinator::{alt, repeat, separated},
    token::one_of,
    Parser, Result,
};

struct Roof {
    tiles: Vec<Vec<RoofTile>>,
}

#[derive(Debug, Clone)]
enum RoofTile {
    Empty,
    Antenna(char),
}

fn main() {
    let roof = parse_input();

    part_1(&roof);
    part_2(&roof);
}

fn part_1(roof: &Roof) {
    // we blatently assume width is consistent and is the same as height
    let roof_diameter = roof.tiles.len() as i8;

    let map_of_antennas = roof
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| match tile {
                    RoofTile::Empty => None,
                    RoofTile::Antenna(antenna_id) => {
                        Some((I8Vec2::new(x as i8, y as i8), antenna_id))
                    }
                })
        })
        .into_group_map_by(|(_, id)| **id);

    let antinode_count = map_of_antennas
        .values()
        .flat_map(|coords| {
            coords.iter().combinations(2).flat_map(|pair| {
                let (coord1, _) = pair[0];
                let (coord2, _) = pair[1];

                let diff = coord1 - coord2;
                let antinode_a = coord1 + diff;
                let antinode_b = coord2 - diff;
                [antinode_a, antinode_b]
            })
        })
        .filter(|vec| {
            vec.cmpge(I8Vec2::ZERO).all() && vec.x < roof_diameter && vec.y < roof_diameter
        })
        .unique()
        .count();

    println!("Amount of antinodes: {}", antinode_count);
}

fn part_2(roof: &Roof) {
    // we blatently assume width is consistent and is the same as height
    let roof_diameter = roof.tiles.len() as i8;
    let min_vec = I8Vec2::ZERO;
    let max_vec = I8Vec2::new(roof_diameter - 1, roof_diameter - 1);

    let map_of_antennas = roof
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| match tile {
                    RoofTile::Empty => None,
                    RoofTile::Antenna(antenna_id) => {
                        Some((I8Vec2::new(x as i8, y as i8), antenna_id))
                    }
                })
        })
        .into_group_map_by(|(_, id)| **id);

    let antinode_count = map_of_antennas
        .values()
        .flat_map(|coords| {
            coords.iter().combinations(2).flat_map(|pair| {
                let (mut coord1, _) = pair[0];
                let (mut coord2, _) = pair[1];

                let diff = coord1 - coord2;
                let mut antinodes = Vec::new();

                // Now the antennas themselves also contain antinodes
                antinodes.push(coord1);
                antinodes.push(coord2);

                // antinodes adding diff foor coord1
                loop {
                    let new_antinode = coord1 + diff;

                    if new_antinode.cmpge(min_vec).all() && new_antinode.cmple(max_vec).all() {
                        antinodes.push(new_antinode);
                        coord1 = new_antinode;
                    } else {
                        break;
                    }
                }

                // antinodes subtracting diff for coord 2
                loop {
                    let new_antinode = coord2 - diff;

                    if new_antinode.cmpge(min_vec).all() && new_antinode.cmple(max_vec).all() {
                        antinodes.push(new_antinode);
                        coord2 = new_antinode;
                    } else {
                        break;
                    }
                }

                antinodes
            })
        })
        .unique()
        .count();

    println!("Amount of antinodes: {}", antinode_count);
}

fn parse_input() -> Roof {
    let mut input = include_str!("../input.txt");
    let tiles = parse_roof(&mut input).unwrap();
    Roof { tiles }
}

fn parse_roof(input: &mut &str) -> Result<Vec<Vec<RoofTile>>> {
    separated(1.., parse_roof_row, line_ending).parse_next(input)
}

fn parse_roof_row(input: &mut &str) -> Result<Vec<RoofTile>> {
    repeat(
        1..,
        alt((
            '.'.map(|_| RoofTile::Empty),
            one_of(('a'..='z', 'A'..='Z', '0'..='9')).map(RoofTile::Antenna),
        )),
    )
    .parse_next(input)
}
