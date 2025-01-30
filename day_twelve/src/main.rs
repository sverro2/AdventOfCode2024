fn main() {
    let input = include_str!("../input.txt");

    let garden = parse_input(input);
    garden
        .iter()
        .take(5)
        .for_each(|row| println!("{:?}", row.iter().map(|a| a.character).collect::<Vec<_>>()));
}

#[derive(Debug)]
struct LocationIndex {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Location {
    index: LocationIndex,
    character: char,
}

fn parse_input(input: &str) -> Vec<Vec<Location>> {
    input
        .split("\n")
        .enumerate()
        .map(|(index_y, line)| {
            line.chars()
                .enumerate()
                .map(|(index_x, character)| Location {
                    index: LocationIndex {
                        x: index_x as i32,
                        y: index_y as i32,
                    },
                    character,
                })
                .collect()
        })
        .collect()
}
