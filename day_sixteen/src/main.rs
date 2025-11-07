use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Not,
};

mod parser;

fn main() {
    let input = parser::parse_input(include_str!("../input.txt"));
    // first_star(&input); // see bottom of file to see first attempt
    first_star_second_try(&input);
    second_star(&input);
}

fn first_star_second_try(input: &AoCInput) {
    if let Some(cost) = dijkstra_with_turn_score(input.start.to_owned(), input.finish, &input.walls)
    {
        println!("Minimum fuel cost to goal: {}", cost.finish_score);
    } else {
        println!("Goal is unreachable");
    }
}

fn second_star(input: &AoCInput) {
    if let Some(scores) =
        dijkstra_with_turn_score(input.start.to_owned(), input.finish, &input.walls)
    {
        let tiles = best_path_tiles(&scores.all, input.start.position, input.finish);
        println!("Amount of tiles on shortest paths: {}", tiles.len());

        // let x_max_index = input.walls.iter().map(|i| i.x).max().unwrap();
        // let y_max_index = input.walls.iter().map(|i| i.y).max().unwrap();

        // (0..=y_max_index).for_each(|y| {
        //     (0..=x_max_index).to_owned().for_each(|x| {
        //         let print_pos = Position { x, y };
        //         if input.walls.contains(&print_pos) {
        //             print!("#");
        //         } else if tiles.contains(&print_pos) {
        //             print!("O");
        //         } else {
        //             print!(" ");
        //         }
        //     });
        //     println!();
        // });
    } else {
        println!("Goal is unreachable");
    }
}

fn dijkstra_with_turn_score(
    start: PlayerState,
    goal: Position,
    walls: &HashSet<Position>,
) -> Option<DijkstraStats> {
    let mut costs: HashMap<PlayerState, u32> = HashMap::new();
    let mut frontier = BinaryHeap::new();

    costs.insert(start.to_owned(), 0);
    frontier.push(Reverse((0, start)));

    while let Some(Reverse((cost, current))) = frontier.pop() {
        if current.position == goal {
            return Some(DijkstraStats {
                finish_score: cost,
                all: costs,
            });
        }

        // Skip if we already found a cheaper path to this state
        if let Some(&existing) = costs.get(&current)
            && cost > existing
        {
            continue;
        }

        for possible_move in get_possible_moves_dijkstra(&current, walls) {
            let next_player_state = possible_move.to_player();
            let next_cost = cost + possible_move.cost;
            if costs.get(&next_player_state).is_none_or(|&c| next_cost < c) {
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
        .filter(|direction| direction != &&player.heading.opposite())
        .filter_map(|direction| {
            let possible_location_to_travel = player.position.travel(direction);
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

fn calculate_move_cost(old_direction: Heading, new_direction: Heading) -> u32 {
    let turn_diff = (old_direction as isize).abs_diff(new_direction as isize);
    let amount_of_turns = turn_diff.min(ALL_HEADINGS.len() - turn_diff);

    // Every step costs 1, every turn 1000
    amount_of_turns as u32 * 1000 + 1
}

type ScoresForPositionMap = HashMap<Position, Vec<(Heading, u32)>>;

fn best_path_tiles(
    costs: &HashMap<PlayerState, u32>,
    start: Position,
    finish: Position,
) -> HashSet<Position> {
    let scores_per_position: ScoresForPositionMap =
        costs
            .iter()
            .fold(HashMap::new(), |mut acc, (player_state, score)| {
                acc.entry(player_state.position)
                    .or_default()
                    .push((player_state.heading, *score));

                acc
            });

    let mut tiles = HashSet::new();
    find_best_path_tiles_by_backtracking(&mut tiles, &scores_per_position, finish, start, None);
    tiles
}

fn find_best_path_tiles_by_backtracking(
    on_shortest_path: &mut HashSet<Position>,
    scores_per_position: &ScoresForPositionMap,
    current_position: Position,
    start_position: Position,
    previous_heading: Option<Heading>,
) {
    // First ensure visited item is stored to set
    on_shortest_path.insert(current_position);

    if current_position == start_position {
        return;
    }

    // Then backtrack each possible shortest path, first found out from
    // what heading the shortest path came from.
    let postition_scores: Vec<_> = scores_per_position
        .get(&current_position)
        .unwrap_or_else(|| panic!(
            "Path cannot be backtracked. Input seems... wrong ({:?}, start: {start_position:?})",
            current_position,
        ))
        .iter()
        .map(|(heading, score)| {
            if let Some(previous_heading) = previous_heading.as_ref()
                && previous_heading != heading
            {
                (heading, score + 1000)
            } else {
                (heading, *score)
            }
        })
        .collect();

    let min_score = postition_scores
        .iter()
        .map(|(_, score)| *score)
        .min()
        .expect("Every position in a shortest path should have a score");

    for (heading, score) in postition_scores {
        let has_min_score = score == min_score;
        let next_position = current_position.travel(&heading.opposite());
        let is_already_checked = on_shortest_path.contains(&next_position);

        if !is_already_checked && has_min_score {
            find_best_path_tiles_by_backtracking(
                on_shortest_path,
                scores_per_position,
                next_position,
                start_position,
                Some(*heading),
            );
        } else {
            continue;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Heading, calculate_move_cost};

    #[test]
    fn north_to_east_cost() {
        let result = calculate_move_cost(Heading::North, Heading::East);

        assert_eq!(result, 1001)
    }

    #[test]
    fn east_to_north_cost() {
        let result = calculate_move_cost(Heading::East, Heading::North);

        assert_eq!(result, 1001)
    }

    #[test]
    fn south_to_north_cost() {
        let result = calculate_move_cost(Heading::North, Heading::South);

        assert_eq!(result, 2001)
    }

    #[test]
    fn north_to_south_cost() {
        let result = calculate_move_cost(Heading::South, Heading::North);

        assert_eq!(result, 2001)
    }

    #[test]
    fn east_to_east_cost() {
        let result = calculate_move_cost(Heading::East, Heading::East);

        assert_eq!(result, 1)
    }

    #[test]
    fn east_to_west_cost() {
        let result = calculate_move_cost(Heading::East, Heading::West);

        assert_eq!(result, 2001)
    }
}

// Just keeping the first depth search attempt arround... Just to remember that doesn't always work well :D
#[allow(dead_code)]
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
    player: &PlayerState,
    finish: Position,
    walls: &HashSet<Position>,
    mut visited: HashSet<Position>,
    current_score: u32,
    lowest_score_to_finish: u32,
) -> u32 {
    let moves = get_possible_moves(player, &visited, walls);
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
            let possible_location_to_travel = player.position.travel(direction);
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

impl Heading {
    fn opposite(&self) -> Self {
        match self {
            Heading::North => Heading::South,
            Heading::East => Heading::West,
            Heading::South => Heading::North,
            Heading::West => Heading::East,
        }
    }
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

struct DijkstraStats {
    finish_score: u32,
    all: HashMap<PlayerState, u32>,
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
