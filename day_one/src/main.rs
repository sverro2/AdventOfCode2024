use std::fs;

// Using Winnow parser in the future?

#[derive(Clone)]
struct InputNumbers {
    left_numbers: Vec<i32>,
    right_numbers: Vec<i32>,
}

fn main() {
    // print string line by line for file "input.txt"
    let parsed_input = fs::read_to_string("day_one/input.txt")
        .unwrap()
        .lines()
        .into_iter()
        .fold(
            InputNumbers {
                left_numbers: vec![],
                right_numbers: vec![],
            },
            |mut parsed, line| {
                let left = line[0..=4].parse::<i32>().unwrap();
                parsed.left_numbers.push(left);

                let right = line[8..=12].parse::<i32>().unwrap();

                parsed.right_numbers.push(right);

                parsed
            },
        );

    // First star
    calculate_distance(parsed_input.to_owned());

    // Second star
    calculate_simularity(parsed_input);
}

fn calculate_distance(mut input: InputNumbers) {
    input.left_numbers.sort();
    input.right_numbers.sort();

    let total_difference: i32 = input
        .left_numbers
        .iter()
        .zip(input.right_numbers)
        .map(|(a, b)| i32::abs(a - b))
        .sum();

    println!("Total difference: {}", total_difference);
}

fn calculate_simularity(input: InputNumbers) {
    let score = input.left_numbers.iter().fold(0, |total, left_number| {
        let encounters_in_right = input
            .right_numbers
            .iter()
            .filter(|right_number| left_number == *right_number)
            .count();

        total + (left_number * encounters_in_right as i32)
    });

    println!("Score: {}", score);
}
