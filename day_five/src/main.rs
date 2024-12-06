use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let mut lines_iterator = input.lines();
    let mut ordering_data: HashMap<i32, HashSet<i32>> = HashMap::new();

    loop {
        let line = lines_iterator.next().unwrap();

        // after an empty line the rest of the input is listed
        if line.is_empty() {
            break;
        }

        // otherwise proces order data
        let (before, after) = line
            .split_once('|')
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();
        ordering_data
            .entry(before)
            .and_modify(|afters| _ = afters.insert(after))
            .or_insert([after].into());
    }

    // Now inport the 'update' data
    let update_list: Vec<Vec<i32>> = lines_iterator
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    let total_middle_levels: i32 = update_list
        .iter()
        .filter_map(|update_line| {
            // Check if update line is ok
            let update_list_length = update_line.len();

            // Checking after first (that one is always ok, as well as the last item)
            let error_found = (1..update_list_length - 1).any(|index_to_check| {
                let Some(required_after_values) = ordering_data.get(&update_line[index_to_check])
                else {
                    return false;
                };
                let previous_values: HashSet<_> = (0..index_to_check)
                    .map(|index| update_line[index])
                    .collect();

                previous_values.intersection(required_after_values).count() != 0
            });

            // Return middle level (because this one must be summed for aoc answer)
            if error_found {
                None
            } else {
                Some(update_line[update_list_length / 2])
            }
        })
        .sum();

    println!("Hello, world! {:#?}", total_middle_levels);
}
