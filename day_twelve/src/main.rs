use std::collections::HashSet;

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

impl std::ops::Add for GardenRegionStats {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        GardenRegionStats {
            plot_count: self.plot_count + other.plot_count,
            fence_count: self.fence_count + other.fence_count,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let garden = Garden::new(parse_input(input));
    let mut already_checked: HashSet<Location> = HashSet::new();

    let total_costs: usize = garden
        .plots
        .iter()
        .map(|row| {
            let row_and_connected_cost: usize = row
                .iter()
                .map(|garden_plot| {
                    if !already_checked.contains(&garden_plot.location) {
                        let stats = calculate_region_fence_price(
                            garden_plot,
                            &garden,
                            &mut already_checked,
                        );
                        stats.plot_count * stats.fence_count
                    } else {
                        0
                    }
                })
                .sum();
            row_and_connected_cost
        })
        .sum();

    println!("The total cost is {total_costs}");
}

const HEADING_COUNT: usize = 4;
const UP: Heading = Heading { x: 0, y: -1 };
const RIGHT: Heading = Heading { x: 1, y: 0 };
const DOWN: Heading = Heading { x: 0, y: 1 };
const LEFT: Heading = Heading { x: -1, y: 0 };
const HEADINGS: [Heading; HEADING_COUNT] = [UP, RIGHT, DOWN, LEFT];

fn calculate_region_fence_price(
    plot: &GardenPlot,
    garden: &Garden,
    already_checked: &mut HashSet<Location>,
) -> GardenRegionStats {
    already_checked.insert(plot.location.to_owned());

    let neighbours_of_same_kind: Vec<&GardenPlot> = HEADINGS
        .iter()
        .filter_map(|heading| garden.find_identical_neighbour(plot, heading))
        .collect();

    let region_stats = GardenRegionStats {
        fence_count: HEADING_COUNT - neighbours_of_same_kind.len(),
        plot_count: 1,
    };

    let undiscovered_neighbours: GardenRegionStats = neighbours_of_same_kind
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

    region_stats + undiscovered_neighbours
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
