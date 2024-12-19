use itertools::Itertools;
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
    let disk_map: Vec<_> = disk_map
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

    let mut usable_space_list: Vec<(usize, UsableSpace)> = disk_map
        .iter()
        .enumerate()
        .skip(1)
        .step_by(2)
        .map(|(index, block)| {
            (
                index,
                match block {
                    // We know empty-index always is emptiness with aoc input
                    DiskMapPart::FileBlock(_) => unreachable!(),
                    DiskMapPart::UsableSpaceBlock(usable_space) => usable_space.to_owned(),
                },
            )
        })
        .collect();

    let mut file_list: Vec<(usize, File)> = disk_map
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(index, block)| {
            (
                index,
                match &block {
                    // We know file-index always contains a file with aoc input
                    DiskMapPart::FileBlock(file) => file.to_owned(),
                    DiskMapPart::UsableSpaceBlock(_) => unreachable!(),
                },
            )
        })
        .collect();

    let mut remaining_usable_spaces_to_check = usable_space_list.iter_mut().collect::<Vec<_>>();

    // for each file, check if it can be put into a preceding empty space
    for (file_index, file) in file_list.iter_mut().rev() {
        let mut usable_index_to_remove: Option<usize> = None;
        if let Some((usable_index, usable_space)) = remaining_usable_spaces_to_check
            .iter_mut()
            .take_while(|(usable_index, _)| *usable_index < *file_index)
            .find(|(_, usable_space)| usable_space.total_free() >= file.length)
        {
            // Add file to usable space block
            usable_space.used_by.push(file.clone());

            // Prepare removal of in future unusable 'usable_index'
            if usable_space.total_free() == 0 {
                usable_index_to_remove = Some(*usable_index);
            }

            // Old file location must now be emptiness, or at least be counted as 0 ;)
            file.file_id = 0;
        }

        // Find and remove index of usable index that is 'full'
        if let Some(index_to_remove) = usable_index_to_remove.and_then(|index| {
            remaining_usable_spaces_to_check
                .iter()
                .position(|(index_in_list, _)| index == *index_in_list)
        }) {
            remaining_usable_spaces_to_check.remove(index_to_remove);
        }
    }

    let filesystem_checksum: u64 = file_list
        .iter()
        // Expand ids in files
        .map(|(_, file)| {
            (0..file.length)
                .map(move |_| file.file_id)
                .collect::<Vec<_>>()
        })
        // And interleave this with the data from the usable spaces (which might contain files as well)
        .interleave(usable_space_list.iter().map(|(_, usable_space)| {
            usable_space
                .used_by
                .iter()
                .flat_map(move |file| (0..file.length).map(move |_| file.file_id))
                // empty space is padded with 0's
                .chain((0..usable_space.total_free()).map(|_| 0))
                .collect::<Vec<_>>()
        }))
        .flatten()
        .enumerate()
        // Then we are calculating checksum, based on expandedindex
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

#[derive(Debug, Clone)]
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

    fn new_file(file_id: usize, length: usize) -> Self {
        Self::FileBlock(File { file_id, length })
    }
}

impl UsableSpace {
    fn total_free(&self) -> usize {
        self.total_capacity - self.used_by.iter().map(|file| file.length).sum::<usize>()
    }
}
