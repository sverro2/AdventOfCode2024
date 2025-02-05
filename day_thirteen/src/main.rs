use glam::I64Vec2;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use winnow::ascii::digit1;
use winnow::combinator::{preceded, repeat};
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::{Parser, Result};

fn main() {
    let crane_configs = parse_crane_configs();

    // Starting with a very simple but inefficient bruteforce
    part_one(&crane_configs);

    // Better.. but still brute forcing things...
    // The approach takes about 3 hours on a powerful computer. Not ideal.
    // Should have used mathematics (linear algebra) to solve this...
    part_two(&crane_configs);
}

fn part_one(crane_configs: &Vec<CraneConfig>) {
    let least_amount_of_tokens_required: i64 = crane_configs
        .par_iter()
        .flat_map(calculate_fewest_tokens_for_price_v1)
        .sum();

    println!("Least amount of tokens required: {least_amount_of_tokens_required}");
}

fn calculate_fewest_tokens_for_price_v1(crane_config: &CraneConfig) -> Option<i64> {
    const BUTTON_A_TOKEN_PRICE: i64 = 3;
    const BUTTON_B_TOKEN_PRICE: i64 = 1;
    const MAX_TIMES_TO_PRESS_BUTTON: i64 = 100;

    (0..MAX_TIMES_TO_PRESS_BUTTON)
        .flat_map(|a| {
            (0..MAX_TIMES_TO_PRESS_BUTTON).filter_map(move |b| {
                if a * crane_config.button_a + b * crane_config.button_b == crane_config.price {
                    Some(a * BUTTON_A_TOKEN_PRICE + b * BUTTON_B_TOKEN_PRICE)
                } else {
                    None
                }
            })
        })
        .min()
}

fn part_two(crane_configs: &Vec<CraneConfig>) {
    const CONFIG_OFFSET: i64 = 10000000000000;
    let least_amount_of_tokens_required: i64 = crane_configs
        .par_iter()
        .map(|config| CraneConfig {
            button_a: config.button_a,
            button_b: config.button_b,
            price: config.price + CONFIG_OFFSET,
        })
        .flat_map(calculate_fewest_tokens_for_price_v2)
        .sum();

    println!("Least amount of tokens required: {least_amount_of_tokens_required}");
}

fn calculate_fewest_tokens_for_price_v2(crane_config: CraneConfig) -> Option<i64> {
    const BUTTON_A_TOKEN_PRICE: i64 = 3;
    const BUTTON_B_TOKEN_PRICE: i64 = 1;

    let max_times_to_press_button_a: i64 =
        max_presses_for_button(crane_config.button_a, crane_config.price);

    // figure out the maximums for both buttons
    let mut min_cost = None;

    // We break out of loops in a way that is ok for current input data.
    // With other data you might actually have to ... not break out of the loops
    'outer: for a_presses in (0..max_times_to_press_button_a).rev() {
        let vec_after_a_presses = a_presses * crane_config.button_a;
        let remaining = crane_config.price - vec_after_a_presses;

        let divided = remaining / crane_config.button_b;

        // Find how many times button be needs can be pressed to get to the price
        if divided.x == divided.y && remaining % crane_config.button_b == I64Vec2::ZERO {
            let b_presses = divided.y;
            let token_cost = a_presses * BUTTON_A_TOKEN_PRICE + b_presses * BUTTON_B_TOKEN_PRICE;

            match min_cost {
                Some(current_min_cost) if token_cost < current_min_cost => {
                    min_cost = Some(token_cost);
                    break 'outer;
                }
                None => min_cost = Some(token_cost),
                _ => (),
            };
        }
    }

    min_cost
}

fn max_presses_for_button(button_vec: I64Vec2, price_vec: I64Vec2) -> i64 {
    let divided = price_vec / button_vec;

    divided.min_element()
}

fn parse_crane_configs() -> Vec<CraneConfig> {
    let mut input = include_str!("../input.txt");
    repeat(1.., parse_crane)
        .parse_next(&mut input)
        .expect("unable to parse input file")
}

fn parse_crane(input: &mut &str) -> Result<CraneConfig> {
    Ok(CraneConfig {
        button_a: parse_next_vector(input)?,
        button_b: parse_next_vector(input)?,
        price: parse_next_vector(input)?,
    })
}

fn parse_next_vector(input: &mut &str) -> Result<I64Vec2> {
    let mut next_number_parser =
        preceded(take_while(1.., |c: char| !c.is_dec_digit()), digit1).try_map(str::parse);

    let next_vec_x = next_number_parser.parse_next(input)?;
    let next_vec_y = next_number_parser.parse_next(input)?;

    Ok(I64Vec2 {
        x: next_vec_x,
        y: next_vec_y,
    })
}

#[derive(Debug, Clone)]
struct CraneConfig {
    button_a: I64Vec2,
    button_b: I64Vec2,
    price: I64Vec2,
}
