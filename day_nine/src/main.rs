use std::iter::successors;

fn main() {
    let input = include_str!("../input_short.txt");

    // Just map all to u8 (except last newline character)
    let disk_map: Vec<u8> = input[..input.len() - 1]
        .chars()
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect();

    part_one(&disk_map);
    part_two(&disk_map);
}

fn part_one(disk_map: &[u8]) {
    let mut file_block_iter_from_start =
        create_file_block_iter(disk_map.iter().copied().step_by(2).enumerate());
    let mut file_block_iter_from_end =
        create_file_block_iter(disk_map.iter().copied().step_by(2).enumerate().rev());

    let mut disk_block_index = 0..;
    let disk_capacity_used_by_files: usize = disk_map.iter().step_by(2).map(|f| *f as usize).sum();

    let filesystem_checksum: u64 = disk_map
        .iter()
        .enumerate()
        .flat_map(|(disk_map_index, count)| {
            let file_block_iter: &mut dyn Iterator<Item = (usize, u8)> = if disk_map_index % 2 == 0
            {
                // if reading file
                &mut file_block_iter_from_start
            } else {
                // if reading free space
                &mut file_block_iter_from_end
            };

            file_block_iter
                .take(*count as usize)
                .map(|(file_id, _)| disk_block_index.next().unwrap_or(0) as u64 * file_id as u64)
                .collect::<Vec<_>>()
        })
        .take(disk_capacity_used_by_files)
        .sum();

    println!("The checksum is (part 1): {}", filesystem_checksum);
}

fn create_file_block_iter(
    mut file_location_iter: impl Iterator<Item = (usize, u8)>,
) -> impl Iterator<Item = (usize, u8)> {
    successors(file_location_iter.next(), move |(file_id, count)| {
        let blocks_remaining_of_file = count - 1;
        if blocks_remaining_of_file > 0 {
            Some((*file_id, blocks_remaining_of_file))
        } else if let Some((next_file_id, next_file_blocks_count)) = file_location_iter.next() {
            Some((next_file_id, next_file_blocks_count))
        } else {
            None
        }
    })
}

#[derive(Debug)]
struct DiskMapPart {
    total_length: usize,
    file: Option<File>,
}

#[derive(Debug)]
struct File {
    file_id: usize,
}

fn part_two(disk_map: &[u8]) {
    let mut disk_map: Vec<_> = disk_map
        .iter()
        .enumerate()
        .map(|(index, length)| {
            if index % 2 == 0 {
                DiskMapPart {
                    total_length: *length as usize,
                    file: Some(File { file_id: index / 2 }),
                }
            } else {
                DiskMapPart {
                    total_length: *length as usize,
                    file: None,
                }
            }
        })
        .collect();

    // for each file, check if it can be put into a preceding empty space
    for file_index in (2..disk_map.len()).step_by(2).rev() {
        if let Some(empty_index) = (1..file_index).step_by(2).find(|&empty_index| {
            disk_map[empty_index].file.is_none()
                && disk_map[empty_index].total_length >= disk_map[file_index].total_length
        }) {
            // Move the file to the empty space
            disk_map[empty_index].file = disk_map[file_index].file.take();
        }
    }

    let filesystem_checksum: u64 = disk_map
        .iter()
        .flat_map(|item| {
            (0..item.total_length).map(|_| item.file.as_ref().map(|f| f.file_id).unwrap_or(0))
        })
        .inspect(|i| println!("{i}"))
        .enumerate()
        .map(|(index, id)| index as u64 * id as u64)
        .sum();

    // Now calculate checksum
    println!("The checksum is (part 2): {}", filesystem_checksum);
}
