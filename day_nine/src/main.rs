use std::iter::successors;

fn main() {
    let input = include_str!("../input.txt");

    // Just map all to u8 (except last newline character)
    let input: Vec<u8> = input[..input.len() - 1]
        .chars()
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect();

    part_one(&input)
}

fn part_one(input: &[u8]) {
    let mut files_from_end = input.iter().copied().step_by(2).enumerate().rev();
    let mut file_from_end = successors(files_from_end.next(), |(file_id, count)| {
        let new_count = count - 1;

        if new_count > 0 {
            Some((*file_id, new_count))
        } else if let Some((next_file_id, next_count)) = files_from_end.next() {
            Some((next_file_id, next_count))
        } else {
            None
        }
    });

    let mut files_from_start = input.iter().copied().step_by(2).enumerate();
    let mut file_from_start = successors(files_from_start.next(), |(file_id, count)| {
        let new_count = count - 1;

        if new_count > 0 {
            Some((*file_id, new_count))
        } else if let Some((next_file_id, next_count)) = files_from_start.next() {
            Some((next_file_id, next_count))
        } else {
            None
        }
    });

    let mut disk_index = 0..;
    let total_file_usage: usize = input.iter().step_by(2).map(|f| *f as usize).sum();

    let a: u64 = input
        .iter()
        .enumerate()
        .flat_map(|(input_index, count)| {
            if input_index % 2 == 0 {
                file_from_start
                    .by_ref()
                    .take(*count as usize)
                    .map(|(file_id, _)| disk_index.next().unwrap() as u64 * file_id as u64)
                    .collect::<Vec<_>>()
            } else {
                file_from_end
                    .by_ref()
                    .take(*count as usize)
                    .map(|(file_id, _)| disk_index.next().unwrap() as u64 * file_id as u64)
                    .collect::<Vec<_>>()
            }
        })
        .take(total_file_usage)
        .sum();

    println!("The checksum is: {}", a);
}
