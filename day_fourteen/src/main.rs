use std::collections::HashSet;

use glam::IVec2;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{separated, separated_pair},
    Parser, Result,
};

const ROOM_WIDE: i32 = 101;
const ROOM_TALL: i32 = 103;
const AMOUNT_OF_SECONDS_SIMULATED: i32 = 100;

#[derive(Default, Debug, PartialEq, Eq)]
struct BotsInQuadrantCount {
    top_left: usize,
    top_right: usize,
    bottom_left: usize,
    bottom_right: usize,
}

impl BotsInQuadrantCount {
    fn increase(&mut self, location: &IVec2) {
        const MIDDLE_HORIZONTALY: i32 = (ROOM_WIDE - 1) / 2;
        const MIDDLE_VERTICALLY: i32 = (ROOM_TALL - 1) / 2;
        let left = location.x < MIDDLE_HORIZONTALY;
        let right = location.x > MIDDLE_HORIZONTALY;
        let top = location.y < MIDDLE_VERTICALLY;
        let bottom = location.y > MIDDLE_VERTICALLY;

        match (top, right, bottom, left) {
            (true, false, false, true) => self.top_left += 1,
            (true, true, false, false) => self.top_right += 1,
            (false, false, true, true) => self.bottom_left += 1,
            (false, true, true, false) => self.bottom_right += 1,
            _ => (),
        }
    }
}

fn main() {
    let parsed_bots = parse_restroom_bots();
    part_1(&parsed_bots);
    part_2(&parsed_bots);
}

fn part_1(bots: &[SecurityBotConfig]) {
    let mut count = BotsInQuadrantCount::default();

    bots.iter()
        .map(|bot| {
            calculate_location_after_simulation(
                bot,
                AMOUNT_OF_SECONDS_SIMULATED,
                ROOM_WIDE,
                ROOM_TALL,
            )
        })
        .for_each(|location| count.increase(&location));

    let safety_factor = count.top_left * count.top_right * count.bottom_left * count.bottom_right;

    println!("{:?}", safety_factor);
}

fn part_2(bots: &[SecurityBotConfig]) {
    // I admit. I didn't really solve it, but wanted to SEE the solution.
    // I saw there was some cyclic action going on, noticed when frames appeared which looked non-random.
    // For my input, the first noisy "image" appeared at 27 seconds. A simular (bit slightly different)
    // picture appeared at 130. This repeated every 103 seconds.
    // with the "script below" I just repeated this until I saw a christmas appearing (pressing ctrl+c) and reading the number
    //
    // a "boring" programming only solution would have been to just see which frame had most connected bots/the least amount of separate groups of bots
    // 10403 was choses as maximum, because I calculated the cycle repeated every 10403 seconds
    // after this cycle you got perfectly identical images.
    const FIRST_NON_RANDOM_LOOKING_OUTPUT_SECONDS: i32 = 27;
    const CYCLE: usize = 103;

    let bot_locations_list: Vec<HashSet<_>> = (FIRST_NON_RANDOM_LOOKING_OUTPUT_SECONDS..10403)
        .step_by(CYCLE)
        .map(|seconds| {
            bots.iter()
                .map(|bot| calculate_location_after_simulation(bot, seconds, ROOM_WIDE, ROOM_TALL))
                .collect()
        })
        .collect();

    bot_locations_list.iter().enumerate().for_each(|(x, list)| {
        (0i32..ROOM_TALL).for_each(|y| {
            (0i32..ROOM_WIDE).for_each(|x| {
                if list.contains(&IVec2 { x, y }) {
                    print!("+")
                } else {
                    print!(" ")
                }
            });
            println!();
        });
        println!(
            "Seconds: {}",
            x * CYCLE + FIRST_NON_RANDOM_LOOKING_OUTPUT_SECONDS as usize
        );
        println!("------------------------------------------");
        std::thread::sleep(std::time::Duration::from_millis(200));

        // sooo lets check when most of the dots are connected and log that number
    });
}

fn calculate_location_after_simulation(
    bot: &SecurityBotConfig,
    iterations: i32,
    x_size: i32,
    y_size: i32,
) -> IVec2 {
    let new_location: IVec2 = bot.location + bot.speed * iterations;

    // Before I discovered rem_euclid, this also works very well!
    // let wrapped_x = ((new_location.x % x_size) + x_size) % x_size;
    // let wrapped_y = ((new_location.y % y_size) + y_size) % y_size;

    new_location.rem_euclid(IVec2 {
        x: x_size,
        y: y_size,
    })
}

fn parse_restroom_bots() -> Vec<SecurityBotConfig> {
    let mut input = include_str!("../input.txt");

    separated(0.., parse_bot, line_ending)
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

#[cfg(test)]
mod test {
    use glam::IVec2;

    use crate::{
        calculate_location_after_simulation, BotsInQuadrantCount, SecurityBotConfig, ROOM_TALL,
        ROOM_WIDE,
    };

    #[test]
    fn test_location_without_wrap() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 5, y: 20 },
            speed: IVec2 { x: 3, y: -3 },
        };

        let expected_sim_location = IVec2 { x: 8, y: 17 };

        let sim_located = calculate_location_after_simulation(&bot, 1, ROOM_WIDE, ROOM_TALL);
        assert_eq!(expected_sim_location, sim_located)
    }

    #[test]
    fn test_location_overflow_wrap() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: ROOM_WIDE,
                y: ROOM_TALL,
            },
        };

        let sim_located = calculate_location_after_simulation(&bot, 1, ROOM_WIDE, ROOM_TALL);
        assert_eq!(bot.location, sim_located)
    }

    #[test]
    fn test_location_overflow_and_more_wrap() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: ROOM_WIDE + 1,
                y: ROOM_TALL + 1,
            },
        };

        let expected_sim_location = IVec2 { x: 1, y: 1 };

        let sim_located = calculate_location_after_simulation(&bot, 1, ROOM_WIDE, ROOM_TALL);
        assert_eq!(expected_sim_location, sim_located)
    }

    #[test]
    fn test_location_overflow_but_more_wrap() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: 2 * ROOM_WIDE + 1,
                y: 2 * ROOM_TALL + 1,
            },
        };

        let expected_sim_location = IVec2 { x: 1, y: 1 };

        let sim_located = calculate_location_after_simulation(&bot, 1, ROOM_WIDE, ROOM_TALL);
        assert_eq!(expected_sim_location, sim_located)
    }

    #[test]
    fn test_location_underflow_wrap() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: -ROOM_WIDE,
                y: -ROOM_TALL,
            },
        };

        let sim_located = calculate_location_after_simulation(&bot, 1, ROOM_WIDE, ROOM_TALL);
        assert_eq!(bot.location, sim_located)
    }

    #[test]
    fn test_location_underflow_and_less_wrap() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: -ROOM_WIDE - 1,
                y: -ROOM_TALL - 1,
            },
        };

        let expected_sim_location = IVec2 {
            x: ROOM_WIDE - 2,
            y: ROOM_TALL - 2,
        };

        let sim_located = calculate_location_after_simulation(&bot, 2, ROOM_WIDE, ROOM_TALL);
        assert_eq!(expected_sim_location, sim_located)
    }

    #[test]
    fn test_location_underflow_but_more_wrap() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: -2 * ROOM_WIDE + 1,
                y: -2 * ROOM_TALL + 1,
            },
        };

        let expected_sim_location = IVec2 { x: 1, y: 1 };

        let sim_located = calculate_location_after_simulation(&bot, 1, ROOM_WIDE, ROOM_TALL);
        assert_eq!(expected_sim_location, sim_located)
    }

    #[test]
    fn test_location_without_wrap_multiple() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 5, y: 30 },
            speed: IVec2 { x: 3, y: -3 },
        };

        let expected_sim_location = IVec2 { x: 35, y: 0 };

        let sim_located = calculate_location_after_simulation(&bot, 10, ROOM_WIDE, ROOM_TALL);
        assert_eq!(expected_sim_location, sim_located)
    }

    #[test]
    fn test_location_overflow_wrap_multiple() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: ROOM_WIDE,
                y: ROOM_TALL,
            },
        };

        let sim_located = calculate_location_after_simulation(&bot, 10, ROOM_WIDE, ROOM_TALL);
        assert_eq!(bot.location, sim_located)
    }

    #[test]
    fn test_location_underflow_wrap_multiple() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 0, y: 0 },
            speed: IVec2 {
                x: -ROOM_WIDE,
                y: -ROOM_TALL,
            },
        };

        let sim_located = calculate_location_after_simulation(&bot, 10, ROOM_WIDE, ROOM_TALL);
        assert_eq!(bot.location, sim_located)
    }

    #[test]
    fn test_location_aoc_example() {
        let bot = SecurityBotConfig {
            location: IVec2 { x: 2, y: 4 },
            speed: IVec2 { x: 2, y: -3 },
        };

        let expected_location = IVec2 { x: 1, y: 3 };

        let sim_located = calculate_location_after_simulation(&bot, 5, 11, 7);
        assert_eq!(expected_location, sim_located)
    }

    #[test]
    fn test_left_top_quadrant() {
        let location = IVec2 { x: 49, y: 50 };
        let mut count = BotsInQuadrantCount::default();
        let expected_count = BotsInQuadrantCount {
            top_left: 1,
            ..Default::default()
        };

        count.increase(&location);

        assert_eq!(expected_count, count);
    }

    #[test]
    fn test_right_top_quadrant() {
        let location = IVec2 { x: 51, y: 50 };
        let mut count = BotsInQuadrantCount::default();
        let expected_count = BotsInQuadrantCount {
            top_right: 1,
            ..Default::default()
        };

        count.increase(&location);

        assert_eq!(expected_count, count);
    }

    #[test]
    fn test_left_bottom_quadrant() {
        let location = IVec2 { x: 49, y: 52 };
        let mut count = BotsInQuadrantCount::default();
        let expected_count = BotsInQuadrantCount {
            bottom_left: 1,
            ..Default::default()
        };

        count.increase(&location);

        assert_eq!(expected_count, count);
    }

    #[test]
    fn test_right_bottom_quadrant() {
        let location = IVec2 { x: 51, y: 52 };
        let mut count = BotsInQuadrantCount::default();
        let expected_count = BotsInQuadrantCount {
            bottom_right: 1,
            ..Default::default()
        };

        count.increase(&location);

        assert_eq!(expected_count, count);
    }

    #[test]
    fn test_quadrant_horizontal_center_ignored() {
        let location = IVec2 { x: 50, y: 0 };
        let mut count = BotsInQuadrantCount::default();
        let expected_count = BotsInQuadrantCount {
            ..Default::default()
        };

        count.increase(&location);

        assert_eq!(expected_count, count);
    }

    #[test]
    fn test_quadrant_vertical_center_ignored() {
        let location = IVec2 { x: 0, y: 51 };
        let mut count = BotsInQuadrantCount::default();
        let expected_count = BotsInQuadrantCount {
            ..Default::default()
        };

        count.increase(&location);

        assert_eq!(expected_count, count);
    }
}
