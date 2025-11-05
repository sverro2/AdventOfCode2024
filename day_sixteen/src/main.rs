use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Not,
};

mod parser;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Heading {
    North,
    East,
    South,
    West,
}

static ALL_HEADINGS: &[Heading] = &[Heading::North, Heading::East, Heading::South, Heading::West];

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn travel(&self, direction: &Heading) -> Self {
        match direction {
            Heading::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Heading::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Heading::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Heading::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct PlayerMove {
    heading: Heading,
    new_position: Position,
    cost: u32,
}

impl PlayerMove {
    pub fn to_player(&self) -> PlayerState {
        PlayerState {
            position: self.new_position,
            heading: self.heading,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Clone)]
struct PlayerState {
    position: Position,
    heading: Heading,
}

struct AoCInput {
    walls: HashSet<Position>,
    start: PlayerState,
    finish: Position,
}

impl AoCInput {
    pub fn new(walls: HashSet<Position>, start: Position, finish: Position) -> Self {
        Self {
            walls,
            // Going to the east is defined in AoC text
            start: PlayerState {
                position: start,
                heading: Heading::East,
            },
            finish,
        }
    }
}

fn main() {
    let input = parser::parse_input(include_str!("../input.txt"));
    // first_star(&input);
    first_star_second_try(&input);
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

#[allow(dead_code)]
fn calculate_path_score(
    player: &PlayerState,
    finish: Position,
    walls: &HashSet<Position>,
    mut visited: HashSet<Position>,
    current_score: u32,
    lowest_score_to_finish: u32,
) -> u32 {
    let moves = get_possible_moves(&player, &visited, walls);
    visited.insert(player.position);

    // This will work, but veeeery slowly, even with the optimalisation that stops searching if totalscore exceeds current minima.
    if player.position == finish {
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
    player: &PlayerState,
    walls: &HashSet<Position>,
    visited: &HashSet<Position>,
) -> Vec<PlayerMove> {
    ALL_HEADINGS
        .iter()
        .filter_map(|direction| {
            let possible_location_to_travel = player.position.travel(&direction);
            if walls.contains(&possible_location_to_travel)
                || visited.contains(&possible_location_to_travel)
            {
                None
            } else {
                Some(PlayerMove {
                    heading: *direction,
                    new_position: possible_location_to_travel,
                    cost: calculate_move_cost(player.heading, *direction),
                })
            }
        })
        .collect()
}

fn calculate_move_cost(old_direction: Heading, new_direction: Heading) -> u32 {
    let old_direction_index = ALL_HEADINGS
        .iter()
        .position(|&direction| direction == old_direction)
        .expect("all directions");

    let new_direction_index = ALL_HEADINGS
        .iter()
        .position(|&direction| direction == new_direction)
        .expect("all directions");

    let turn_diff = old_direction_index.abs_diff(new_direction_index);
    let amount_of_turns = turn_diff.min(ALL_HEADINGS.len() - turn_diff);

    // Every step costs 1, every turn 1000
    amount_of_turns as u32 * 1000 + 1
}

fn first_star_second_try(input: &AoCInput) {
    if let Some(cost) = dijkstra_with_turn_score(input.start.to_owned(), input.finish, &input.walls)
    {
        println!("Minimum fuel cost to goal: {}", cost);
    } else {
        println!("Goal is unreachable");
    }
}

fn dijkstra_with_turn_score(
    start: PlayerState,
    goal: Position,
    walls: &HashSet<Position>,
) -> Option<u32> {
    let mut costs: HashMap<PlayerState, u32> = HashMap::new();
    let mut frontier = BinaryHeap::new();

    costs.insert(start.to_owned(), 0);
    frontier.push(Reverse((0, start)));

    while let Some(Reverse((cost, current))) = frontier.pop() {
        if current.position == goal {
            return Some(cost);
        }

        // Skip if we already found a cheaper path to this state
        if let Some(&existing) = costs.get(&current) {
            if cost > existing {
                continue;
            }
        }

        for possible_move in get_possible_moves_dijkstra(&current, walls) {
            let next_player_state = possible_move.to_player();
            let next_cost = cost + possible_move.cost;
            if costs
                .get(&next_player_state)
                .map_or(true, |&c| next_cost < c)
            {
                costs.insert(next_player_state.to_owned(), next_cost);
                frontier.push(Reverse((next_cost, next_player_state)));
            }
        }
    }

    None
}

fn get_possible_moves_dijkstra(player: &PlayerState, walls: &HashSet<Position>) -> Vec<PlayerMove> {
    ALL_HEADINGS
        .iter()
        .filter_map(|direction| {
            let possible_location_to_travel = player.position.travel(&direction);
            walls
                .contains(&possible_location_to_travel)
                .not()
                .then(|| PlayerMove {
                    heading: *direction,
                    new_position: possible_location_to_travel,
                    cost: calculate_move_cost(player.heading, *direction),
                })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::calculate_move_cost;

    #[test]
    fn north_to_east_cost() {
        let result = calculate_move_cost(crate::Heading::North, crate::Heading::East);

        assert_eq!(result, 1001)
    }

    #[test]
    fn east_to_north_cost() {
        let result = calculate_move_cost(crate::Heading::East, crate::Heading::North);

        assert_eq!(result, 1001)
    }

    #[test]
    fn south_to_north_cost() {
        let result = calculate_move_cost(crate::Heading::North, crate::Heading::South);

        assert_eq!(result, 2001)
    }

    #[test]
    fn north_to_south_cost() {
        let result = calculate_move_cost(crate::Heading::South, crate::Heading::North);

        assert_eq!(result, 2001)
    }

    #[test]
    fn east_to_east_cost() {
        let result = calculate_move_cost(crate::Heading::East, crate::Heading::East);

        assert_eq!(result, 1)
    }

    #[test]
    fn east_to_west_cost() {
        let result = calculate_move_cost(crate::Heading::East, crate::Heading::West);

        assert_eq!(result, 2001)
    }
}
