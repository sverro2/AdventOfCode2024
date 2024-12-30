use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input_stones = parse_input();
    part_1(&input_stones);
    part_2(&input_stones);
}

fn part_1(stones: &[u64]) {
    let times_blinking = 25;
    let amount_of_stones: u64 = stones
        .par_iter()
        .map(|stone| count_stones_recursively(*stone, times_blinking))
        .sum();

    println!("Amount of stones: {}", amount_of_stones);
}

fn count_stones_recursively(stone: u64, remaining: usize) -> u64 {
    if remaining == 0 {
        1
    } else {
        let next_remaining = remaining - 1;
        if stone == 0 {
            count_stones_recursively(1, next_remaining)
        } else if check_stone_has_even_number_of_digits(stone) {
            let (left_stone, right_stone) = split_stone_into_halves(stone);
            count_stones_recursively(left_stone, next_remaining)
                + count_stones_recursively(right_stone, next_remaining)
        } else {
            count_stones_recursively(stone * 2024, next_remaining)
        }
    }
}

fn part_2(stones: &[u64]) {
    let times_blinking_per_chunk: usize = 25;
    let depth = 3; // 3 * 25 = 75 blinks total

    let counted_stones = find_stones_chunked(stones, depth, times_blinking_per_chunk);
    println!("Stones counted: {}", counted_stones);
}

fn find_stones_chunked(stones: &[u64], remaining_chunks: usize, blinks_per_chunk: usize) -> u64 {
    let amount_of_stones = stones.iter().copied().counts();

    if remaining_chunks == 0 {
        amount_of_stones.values().map(|i| *i as u64).sum()
    } else {
        amount_of_stones
            .par_iter()
            .fold(
                || 0,
                |acc, (stone, multiplier)| {
                    acc + find_stones_chunked(
                        &find_stones_recursively(*stone, blinks_per_chunk),
                        remaining_chunks - 1,
                        blinks_per_chunk,
                    ) * *multiplier as u64
                },
            )
            .sum()
    }
}

fn find_stones_recursively(stone: u64, remaining_blinks: usize) -> Vec<u64> {
    if remaining_blinks == 0 {
        vec![stone]
    } else {
        let next_remaining = remaining_blinks - 1;

        if stone == 0 {
            find_stones_recursively(1, next_remaining)
        } else if check_stone_has_even_number_of_digits(stone) {
            let (left_stone, right_stone) = split_stone_into_halves(stone);

            let mut stones = find_stones_recursively(left_stone, next_remaining);
            stones.extend(find_stones_recursively(right_stone, next_remaining));
            stones
        } else {
            find_stones_recursively(stone * 2024, next_remaining)
        }
    }
}

fn check_stone_has_even_number_of_digits(number: u64) -> bool {
    number.ilog10() % 2 == 1
}

fn split_stone_into_halves(number: u64) -> (u64, u64) {
    let number_as_string = number.to_string();
    let half_length = number_as_string.len() / 2;
    let (left, right) = number_as_string.split_at(half_length);
    (left.parse().unwrap(), right.parse().unwrap())
}

fn parse_input() -> Vec<u64> {
    let input = include_str!("../input.txt");

    // Get all input except for the last 'newline' character
    input[0..input.len() - 1]
        .split(' ')
        .map(|stone| stone.parse().unwrap())
        .collect()
}
