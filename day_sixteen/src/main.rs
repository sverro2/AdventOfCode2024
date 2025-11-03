use std::collections::HashSet;

mod parser;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
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

impl PlayerMove {
    pub fn to_player(&self) -> Player {
        Player {
            location: self.new_location,
            direction: self.direction,
        }
    }
}

#[derive(Debug)]
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
    let score = calculate_path_score(
        &input.start,
        input.finish,
        &input.walls,
        HashSet::new(),
        0,
        u32::MAX,
    );

    println!("Lowest score is {score:?}");
}

fn calculate_path_score(
    player: &Player,
    finish: Location,
    walls: &HashSet<Location>,
    mut visited: HashSet<Location>,
    current_score: u32,
    lowest_score_to_finish: u32,
) -> u32 {
    let moves = get_possible_moves(&player, &visited, walls);
    visited.insert(player.location);

    // This will work, but veeeery slowly
    if player.location == finish {
        current_score
    } else {
        moves.iter().fold(
            lowest_score_to_finish,
            |lowest_score_to_finish: u32, m: &PlayerMove| {
                let next_chamber_score = current_score + m.cost;

                if lowest_score_to_finish < next_chamber_score {
                    lowest_score_to_finish
                } else {
                    let score_to_finish = calculate_path_score(
                        &m.to_player(),
                        finish,
                        walls,
                        visited.to_owned(),
                        next_chamber_score,
                        lowest_score_to_finish,
                    );

                    lowest_score_to_finish.min(score_to_finish)
                }
            },
        )
    }
}

fn get_possible_moves(
    player: &Player,
    walls: &HashSet<Location>,
    visited: &HashSet<Location>,
) -> Vec<PlayerMove> {
    ALL_DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let possible_location_to_travel = player.location.travel(&direction);
            if walls.contains(&possible_location_to_travel)
                || visited.contains(&possible_location_to_travel)
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

    let turn_diff = old_direction_index.abs_diff(new_direction_index);
    let amount_of_turns = turn_diff.min(ALL_DIRECTIONS.len() - turn_diff);

    // Every step costs 1, every turn 1000
    amount_of_turns as u32 * 1000 + 1
}

#[cfg(test)]
mod test {
    use crate::calculate_move_cost;

    #[test]
    fn north_to_east_cost() {
        let result = calculate_move_cost(crate::Direction::North, crate::Direction::East);

        assert_eq!(result, 1001)
    }

    #[test]
    fn east_to_north_cost() {
        let result = calculate_move_cost(crate::Direction::East, crate::Direction::North);

        assert_eq!(result, 1001)
    }

    #[test]
    fn south_to_north_cost() {
        let result = calculate_move_cost(crate::Direction::North, crate::Direction::South);

        assert_eq!(result, 2001)
    }

    #[test]
    fn north_to_south_cost() {
        let result = calculate_move_cost(crate::Direction::South, crate::Direction::North);

        assert_eq!(result, 2001)
    }

    #[test]
    fn east_to_east_cost() {
        let result = calculate_move_cost(crate::Direction::East, crate::Direction::East);

        assert_eq!(result, 1)
    }

    #[test]
    fn east_to_west_cost() {
        let result = calculate_move_cost(crate::Direction::East, crate::Direction::West);

        assert_eq!(result, 2001)
    }
}
