use std::collections::HashSet;

use itertools::Itertools;
mod part_one;
mod part_two;

fn main() {
    let input = include_str!("../input.txt");

    let garden = Garden::new(parse_input(input));
    part_one(&garden);
    part_two(&garden);
}

fn part_one(garden: &Garden) {
    let mut already_checked: HashSet<Location> = HashSet::new();

    let total_costs: usize = garden
        .plots
        .iter()
        .map(|row| {
            let connected_with_row_cost: usize = row
                .iter()
                .map(|garden_plot| {
                    if !already_checked.contains(&garden_plot.location) {
                        let stats =
                            calculate_region_fence_price(garden_plot, garden, &mut already_checked);
                        stats.plot_count * stats.fence_count
                    } else {
                        0
                    }
                })
                .sum();
            connected_with_row_cost
        })
        .sum();

    println!("The total cost is {total_costs}");
}

fn calculate_region_fence_price(
    plot: &GardenPlot,
    garden: &Garden,
    already_checked: &mut HashSet<Location>,
) -> GardenRegionStats {
    already_checked.insert(plot.location.to_owned());

    let neighbours_of_same_kind: Vec<&GardenPlot> = HEADING_OPTIONS
        .iter()
        .filter_map(|heading| garden.find_identical_neighbour(plot, heading))
        .collect();

    let region_stats = GardenRegionStats {
        fence_count: HEADING_COUNT - neighbours_of_same_kind.len(),
        plot_count: 1,
    };

    let undiscovered_neighbours_stats: GardenRegionStats = neighbours_of_same_kind
        .iter()
        .filter_map(|neightbour| {
            if !already_checked.contains(&neightbour.location) {
                Some(calculate_region_fence_price(
                    neightbour,
                    garden,
                    already_checked,
                ))
            } else {
                None
            }
        })
        .fold(
            GardenRegionStats {
                plot_count: 0,
                fence_count: 0,
            },
            |acc, i| i + acc,
        );

    region_stats + undiscovered_neighbours_stats
}

fn part_two(garden: &Garden) {
    let mut already_checked: HashSet<Location> = HashSet::new();

    let total_costs: usize = garden
        .plots
        .iter()
        .map(|row| {
            let connected_with_row_cost: usize = row
                .iter()
                .map(|garden_plot| {
                    if !already_checked.contains(&garden_plot.location) {
                        let stats = calculate_required_fences_for_region(
                            garden_plot,
                            garden,
                            &mut already_checked,
                        );

                        // Calculate discounted price for fences in all fence directions
                        let total_discounted_fences: usize = HEADING_OPTIONS
                            .iter()
                            .map(FencePostion::fence_for)
                            .map(|direction| {
                                calculate_discounted_fences_count_in_direction(
                                    &stats.fences,
                                    direction,
                                )
                            })
                            .sum();

                        // We still calculate price by multiplying these values
                        stats.plot_count * total_discounted_fences
                    } else {
                        0
                    }
                })
                .sum();
            connected_with_row_cost
        })
        .sum();

    println!("The total cost is {total_costs}");
}

fn calculate_required_fences_for_region(
    plot: &GardenPlot,
    garden: &Garden,
    already_checked: &mut HashSet<Location>,
) -> GardenRegionStatsV2 {
    already_checked.insert(plot.location.to_owned());

    let (identical_neightbours, required_fences): (Vec<&GardenPlot>, Vec<Fence>) = HEADING_OPTIONS
        .iter()
        .fold((vec![], vec![]), |(mut neighbours, mut fences), heading| {
            if let Some(plot) = garden.find_identical_neighbour(plot, heading) {
                neighbours.push(plot);
            } else {
                fences.push(Fence {
                    plot_location: plot.location.to_owned(),
                    position: FencePostion::fence_for(heading),
                });
            }
            (neighbours, fences)
        });

    let region_stats = GardenRegionStatsV2 {
        plot_count: 1,
        fences: required_fences,
    };

    let undiscovered_neighbours_stats: GardenRegionStatsV2 = identical_neightbours
        .iter()
        .filter_map(|neightbour| {
            if !already_checked.contains(&neightbour.location) {
                Some(calculate_required_fences_for_region(
                    neightbour,
                    garden,
                    already_checked,
                ))
            } else {
                None
            }
        })
        .fold(
            GardenRegionStatsV2 {
                plot_count: 0,
                fences: vec![],
            },
            |acc, i| i + acc,
        );

    region_stats + undiscovered_neighbours_stats
}

fn calculate_discounted_fences_count_in_direction(
    fences: &[Fence],
    fence_position: FencePostion,
) -> usize {
    let mut direction_facing_fence_count = fences
        .iter()
        .filter(|i| i.position == fence_position)
        .into_group_map_by(|i| match fence_position {
            FencePostion::North | FencePostion::South => i.plot_location.y,
            FencePostion::East | FencePostion::West => i.plot_location.x,
        });

    direction_facing_fence_count
        .values_mut()
        .for_each(|north_fences| {
            north_fences.sort_by_key(|f| match fence_position {
                FencePostion::North | FencePostion::South => f.plot_location.x,
                FencePostion::East | FencePostion::West => f.plot_location.y,
            })
        });

    // now count connected fences
    let north_facing_count: usize = direction_facing_fence_count
        .values()
        .map(|fences| {
            fences
                .iter()
                .tuple_windows()
                .fold(1, |acc, (fence, next_fence)| {
                    let diff = match fence_position {
                        FencePostion::North | FencePostion::South => {
                            next_fence.plot_location.x - fence.plot_location.x
                        }
                        FencePostion::East | FencePostion::West => {
                            next_fence.plot_location.y - fence.plot_location.y
                        }
                    };

                    if diff == 1 {
                        acc
                    } else {
                        acc + 1
                    }
                })
        })
        .sum();

    north_facing_count
}

fn parse_input(input: &str) -> Vec<Vec<GardenPlot>> {
    input
        .split("\n")
        .enumerate()
        .map(|(index_y, line)| {
            line.chars()
                .enumerate()
                .map(|(index_x, character)| GardenPlot {
                    location: Location {
                        x: index_x,
                        y: index_y,
                    },
                    character,
                })
                .collect()
        })
        .collect()
}

const HEADING_COUNT: usize = 4;
const GO_NORTH: Heading = Heading { x: 0, y: -1 };
const GO_EAST: Heading = Heading { x: 1, y: 0 };
const GO_SOUTH: Heading = Heading { x: 0, y: 1 };
const GO_WEST: Heading = Heading { x: -1, y: 0 };
const HEADING_OPTIONS: [Heading; HEADING_COUNT] = [GO_NORTH, GO_EAST, GO_SOUTH, GO_WEST];

#[derive(PartialEq, Eq)]
enum FencePostion {
    North,
    East,
    South,
    West,
}

impl FencePostion {
    fn fence_for(heading: &Heading) -> Self {
        match *heading {
            GO_NORTH => Self::North,
            GO_EAST => Self::East,
            GO_SOUTH => Self::South,
            GO_WEST => Self::West,
            _ => unreachable!(),
        }
    }
}

struct Garden {
    plots: Vec<Vec<GardenPlot>>,
    size: usize,
}

impl Garden {
    fn new(plots: Vec<Vec<GardenPlot>>) -> Self {
        Self {
            size: plots.len(),
            plots,
        }
    }

    fn find_identical_neighbour<'a>(
        &'a self,
        plot: &GardenPlot,
        heading: &Heading,
    ) -> Option<&'a GardenPlot> {
        let next_x_unchecked = plot.location.x as i32 + heading.x;
        let next_x = (next_x_unchecked >= 0 && next_x_unchecked < self.size as i32)
            .then_some(next_x_unchecked as usize)?;

        let next_y_unchecked = plot.location.y as i32 + heading.y;
        let next_y = (next_y_unchecked >= 0 && next_y_unchecked < self.size as i32)
            .then_some(next_y_unchecked as usize)?;

        let neighbouring_plot = self.plots.get(next_y).and_then(|r| r.get(next_x))?;

        if neighbouring_plot.character == plot.character {
            Some(neighbouring_plot)
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq)]
struct Heading {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct GardenPlot {
    location: Location,
    character: char,
}

struct GardenRegionStats {
    plot_count: usize,
    fence_count: usize,
}

struct GardenRegionStatsV2 {
    plot_count: usize,
    fences: Vec<Fence>,
}

struct Fence {
    plot_location: Location,
    position: FencePostion,
}

impl std::ops::Add for GardenRegionStats {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            plot_count: self.plot_count + other.plot_count,
            fence_count: self.fence_count + other.fence_count,
        }
    }
}

impl std::ops::Add for GardenRegionStatsV2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut combined_fences = self.fences;
        combined_fences.extend(other.fences);
        Self {
            plot_count: self.plot_count + other.plot_count,
            fences: combined_fences,
        }
    }
}
