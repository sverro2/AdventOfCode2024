use std::collections::HashMap;

// Using Winnow parser in the future?

struct InputNumbers {
    list_one_numbers: Vec<i32>,
    list_two_numbers: Vec<i32>,
    list_two_counted_occurances: HashMap<i32, usize>,
}

fn main() {
    let start = std::time::Instant::now();

    let parsed_input = parse_aoc_input();

    // First star
    let distance = calculate_distance(&parsed_input);

    // Second star
    let simularity = calculate_simularity(&parsed_input);

    // Record the amount of time it took to run the program
    let end = std::time::Instant::now();
    println!("Time elapsed: {:?}", end - start);

    println!("Calculated distance: {distance}");
    println!("Similarity value: {simularity}");
}

fn parse_aoc_input() -> InputNumbers {
    let mut parsed_input = include_str!("../input.txt").lines().fold(
        InputNumbers {
            list_one_numbers: vec![],
            list_two_numbers: vec![],
            list_two_counted_occurances: HashMap::new(),
        },
        |mut parsed, line| {
            let left = line[0..=4].parse::<i32>().unwrap();
            parsed.list_one_numbers.push(left);

            let right = line[8..=12].parse::<i32>().unwrap();
            parsed.list_two_numbers.push(right);

            parsed
                .list_two_counted_occurances
                .entry(right)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            parsed
        },
    );

    parsed_input.list_one_numbers.sort();
    parsed_input.list_two_numbers.sort();

    parsed_input
}

fn calculate_distance(input: &InputNumbers) -> i32 {
    let total_difference: i32 = input
        .list_one_numbers
        .iter()
        .zip(&input.list_two_numbers)
        .map(|(a, b)| i32::abs(a - b))
        .sum();

    total_difference
}

fn calculate_simularity(input: &InputNumbers) -> i32 {
    let score: i32 = input
        .list_one_numbers
        .iter()
        .map(|left_number| {
            let encounters_in_right = input
                .list_two_counted_occurances
                .get(left_number)
                .unwrap_or(&0);

            left_number * *encounters_in_right as i32
        })
        .sum();

    score
}
