use std::iter::successors;

fn main() {
    let input = include_str!("../input.txt");

    // Just map all to u8 (except last newline character)
    let disk_map: Vec<u8> = input[..input.len() - 1]
        .chars()
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect();

    part_one(&disk_map);

    // Might be interesting to see if a linked-list would perform better?
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

fn part_two(disk_map: &[u8]) {
    let mut disk_map: Vec<_> = disk_map
        .iter()
        .enumerate()
        .map(|(index, length)| {
            if index % 2 == 0 {
                DiskMapPart::new_file(index / 2, *length as usize)
            } else {
                DiskMapPart::new_empty(*length as usize)
            }
        })
        .collect();

    let mut empty_space_indices: Vec<usize> = (1..disk_map.len()).step_by(2).collect();
    let rev_file_indices: Vec<usize> = (2..disk_map.len()).step_by(2).rev().collect();

    // for each file, check if it can be put into a preceding empty space
    for file_index in rev_file_indices {
        if let Some(&empty_index) = empty_space_indices
            .iter()
            .take_while(|&&index| index < file_index)
            .find(|&&empty_index| {
                matches!(
                    (&disk_map[empty_index], &disk_map[file_index]),
                    (DiskMapPart::UsableSpaceBlock(usable_space), DiskMapPart::FileBlock(file))
                    if usable_space.total_free() >= file.length
                )
            })
        {
            let DiskMapPart::FileBlock(file_to_move) = &disk_map[file_index] else {
                // We know file-index always is a file with aoc input
                unreachable!()
            };

            let file_to_move = file_to_move.clone();

            let DiskMapPart::UsableSpaceBlock(usable_space) = &mut disk_map[empty_index] else {
                // We know empty-index always is emptiness with aoc input
                unreachable!()
            };

            // Add file to usable space block
            usable_space.used_by.push(file_to_move.clone());

            // Make sure we don't try to use the usable-space block
            // when it is all filled up now
            if usable_space.total_free() == 0 {
                empty_space_indices.remove(
                    empty_space_indices
                        .iter()
                        .position(|&i| i == empty_index)
                        .unwrap(),
                );
            }

            // Old file location must now be emptiness.
            disk_map[file_index] = DiskMapPart::new_empty_from(&file_to_move);
        }
    }

    let filesystem_checksum: u64 = disk_map
        .iter()
        .flat_map(|item| -> Box<dyn Iterator<Item = usize>> {
            // We are expanding each item on disk, even empty spaces
            match item {
                DiskMapPart::FileBlock(file) => {
                    Box::new((0..file.length).map(move |_| file.file_id))
                }
                DiskMapPart::UsableSpaceBlock(usable_space) => Box::new(
                    usable_space
                        .used_by
                        .iter()
                        .flat_map(move |file| (0..file.length).map(move |_| file.file_id))
                        .chain((0..usable_space.total_free()).map(|_| 0)),
                ),
            }
        })
        .enumerate()
        .map(|(index, id)| index as u64 * id as u64)
        .sum();

    // Now calculate checksum
    println!("The checksum is (part 2): {}", filesystem_checksum);
}

#[derive(Debug)]
enum DiskMapPart {
    FileBlock(File),
    UsableSpaceBlock(UsableSpace),
}

#[derive(Debug, Clone)]
struct File {
    file_id: usize,
    length: usize,
}

#[derive(Debug)]
struct UsableSpace {
    total_capacity: usize,
    used_by: Vec<File>,
}

impl DiskMapPart {
    fn new_empty(length: usize) -> Self {
        Self::UsableSpaceBlock(UsableSpace {
            total_capacity: length,
            used_by: vec![],
        })
    }

    fn new_empty_from(file: &File) -> Self {
        Self::UsableSpaceBlock(UsableSpace {
            total_capacity: file.length,
            used_by: vec![],
        })
    }

    fn new_file(file_id: usize, length: usize) -> Self {
        Self::FileBlock(File { file_id, length })
    }
}

impl UsableSpace {
    fn total_free(&self) -> usize {
        self.total_capacity - self.used_by.iter().map(|file| file.length).sum::<usize>()
    }
}
