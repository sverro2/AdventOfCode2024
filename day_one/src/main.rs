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
    let distance_sum = calculate_distance(&parsed_input);

    // Second star
    let simularity_score = calculate_simularity(&parsed_input);

    // Record the amount of time it took to run the program
    println!("Time elapsed: {:?}", start.elapsed());

    println!("Calculated distance: {distance_sum}");
    println!("Similarity value: {simularity_score}");
}

fn parse_aoc_input() -> InputNumbers {
    let mut parsed_input = include_str!("../input.txt").lines().fold(
        InputNumbers {
            list_one_numbers: vec![],
            list_two_numbers: vec![],
            list_two_counted_occurances: HashMap::new(),
        },
        |mut parsed, line| {
            let list_one_number = line[..5].parse::<i32>().unwrap();
            parsed.list_one_numbers.push(list_one_number);

            let list_two_number = line[8..].parse::<i32>().unwrap();
            parsed.list_two_numbers.push(list_two_number);

            parsed
                .list_two_counted_occurances
                .entry(list_two_number)
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
    let simularity_score: i32 = input
        .list_one_numbers
        .iter()
        .filter_map(|list_one_number| {
            input
                .list_two_counted_occurances
                .get(list_one_number)
                .map(|occurances| list_one_number * *occurances as i32)
        })
        .sum();

    simularity_score
}
