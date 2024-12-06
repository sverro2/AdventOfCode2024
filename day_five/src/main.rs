use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

type OrderingRules = HashMap<i32, HashSet<i32>>;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let (_, update_list) = aoc_input();

    let total_middle_levels: i32 = update_list
        .iter()
        .filter_map(|update_line| {
            let update_list_length = update_line.len();

            let error_found = !update_line.is_sorted();

            // Return middle level (because this one must be summed for aoc answer)
            if error_found {
                None
            } else {
                Some(update_line[update_list_length / 2].number)
            }
        })
        .sum();

    println!("Total that needed sorting! {:#?}", total_middle_levels);
}

fn part_two() {
    let (_, update_list) = aoc_input();

    let total_middle_levels: i32 = update_list
        .iter()
        .filter_map(|update_line| {
            let update_list_length = update_line.len();

            let error_found = !update_line.is_sorted();

            // Return middle level (because this one must be summed for aoc answer)
            if error_found {
                let mut sorted_line = update_line.clone();
                sorted_line.sort();
                Some(sorted_line[update_list_length / 2].number)
            } else {
                None
            }
        })
        .sum();

    println!("Total that needed sorting! {:#?}", total_middle_levels);
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct UpdateNumber {
    number: i32,
}

impl PartialOrd for UpdateNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UpdateNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (rules, _) = aoc_input();

        if let Some(afters) = rules.get(&self.number) {
            if afters.contains(&other.number) {
                return std::cmp::Ordering::Less;
            }
        }

        if let Some(afters) = rules.get(&other.number) {
            if afters.contains(&self.number) {
                return std::cmp::Ordering::Greater;
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl From<i32> for UpdateNumber {
    fn from(number: i32) -> Self {
        Self { number }
    }
}

fn aoc_input() -> &'static (OrderingRules, Vec<Vec<UpdateNumber>>) {
    static INPUT: OnceLock<(OrderingRules, Vec<Vec<UpdateNumber>>)> = OnceLock::new();
    INPUT.get_or_init(|| {
        let input = include_str!("../input.txt");

        let mut lines_iterator = input.lines();
        let mut ordering_rules: OrderingRules = HashMap::new();

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
            ordering_rules
                .entry(before)
                .and_modify(|afters| _ = afters.insert(after))
                .or_insert([after].into());
        }

        // Now inport the 'update' data
        let update_list: Vec<Vec<UpdateNumber>> = lines_iterator
            .map(|line| {
                line.split(',')
                    .map(|v| v.parse::<i32>().unwrap().into())
                    .collect()
            })
            .collect();

        (ordering_rules, update_list)
    })
}
