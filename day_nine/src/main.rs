use std::iter::successors;

fn main() {
    let input = include_str!("../input.txt");

    // Just map all to u8 (except last newline character)
    let disk_map: Vec<u8> = input[..input.len() - 1]
        .chars()
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect();

    part_one(&disk_map)
}

fn part_one(disk_map: &[u8]) {
    fn create_file_iter(
        mut file_location_iter: impl Iterator<Item = (usize, u8)>,
    ) -> impl Iterator<Item = (usize, u8)> {
        successors(file_location_iter.next(), move |(file_id, count)| {
            let new_count = count - 1;
            if new_count > 0 {
                Some((*file_id, new_count))
            } else if let Some((next_file_id, next_count)) = file_location_iter.next() {
                Some((next_file_id, next_count))
            } else {
                None
            }
        })
    }

    let mut file_iter_from_start =
        create_file_iter(disk_map.iter().copied().step_by(2).enumerate());
    let mut file_iter_from_end =
        create_file_iter(disk_map.iter().copied().step_by(2).enumerate().rev());

    let mut disk_block_index = 0..;
    let disk_capacity_used_by_files: usize = disk_map.iter().step_by(2).map(|f| *f as usize).sum();

    let filesystem_checksum: u64 = disk_map
        .iter()
        .enumerate()
        .flat_map(|(disk_map_index, count)| {
            let iter: &mut dyn Iterator<Item = (usize, u8)> = if disk_map_index % 2 == 0 {
                // if reading file
                &mut file_iter_from_start
            } else {
                // if reading free space
                &mut file_iter_from_end
            };

            iter.take(*count as usize)
                .map(|(file_id, _)| disk_block_index.next().unwrap_or(0) as u64 * file_id as u64)
                .collect::<Vec<_>>()
        })
        .take(disk_capacity_used_by_files)
        .sum();

    println!("The checksum is: {}", filesystem_checksum);
}
