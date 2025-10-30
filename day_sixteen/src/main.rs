use std::collections::HashSet;

mod parser;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

static ALL_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn travel(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct PlayerMove {
    direction: Direction,
    new_location: Location,
    cost: u32,
}

struct Player {
    location: Location,
    direction: Direction,
}

struct AoCInput {
    walls: HashSet<Location>,
    start: Player,
    finish: Location,
}

impl AoCInput {
    pub fn new(walls: HashSet<Location>, start: Location, finish: Location) -> Self {
        Self {
            walls,
            // Going to the east is defined in AoC text
            start: Player {
                location: start,
                direction: Direction::East,
            },
            finish,
        }
    }
}

fn main() {
    let input = parser::parse_input(include_str!("../input.txt"));
    first_star(&input);
}

fn first_star(input: &AoCInput) {
    // println!("So the possible directions are: {possible_directions:?}");
    calculate_path_score(&input.start, HashSet::new(), &input.walls);
}

fn calculate_path_score(
    player: &Player,
    visited: HashSet<Location>,
    walls: &HashSet<Location>,
) -> Option<u32> {
    let moves = get_possible_moves(&player, &visited, walls);
    println!("{moves:?}");
    todo!()
}

fn get_possible_moves(
    player: &Player,
    visited: &HashSet<Location>,
    walls: &HashSet<Location>,
) -> Vec<PlayerMove> {
    ALL_DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let possible_location_to_travel = player.location.travel(&direction);
            if walls.contains(&possible_location_to_travel)
                | visited.contains(&possible_location_to_travel)
            {
                None
            } else {
                Some(PlayerMove {
                    direction: *direction,
                    new_location: possible_location_to_travel,
                    cost: calculate_move_cost(player.direction, *direction),
                })
            }
        })
        .collect()
}

fn calculate_move_cost(old_direction: Direction, new_direction: Direction) -> u32 {
    let old_direction_index = ALL_DIRECTIONS
        .iter()
        .position(|&direction| direction == old_direction)
        .expect("all directions");

    let new_direction_index = ALL_DIRECTIONS
        .iter()
        .position(|&direction| direction == new_direction)
        .expect("all directions");

    let turns_required = if new_direction_index >= old_direction_index {
        new_direction_index - old_direction_index
    } else {
        ALL_DIRECTIONS.len() - old_direction_index + new_direction_index
    };

    // Every step costs 1, every turn 1000
    turns_required as u32 * 1000 + 1
}
