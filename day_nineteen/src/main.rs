use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};
use rayon::prelude::*;

#[derive(Debug)]
struct AOCInput {
    towel_patterns: Vec<String>,
    designs: Vec<String>,
}

fn main() {
    let input = read_input().expect("Not valid AOC input");
    first_star(&input);
    second_star(&input);
}

fn first_star(input: &AOCInput) {
    println!("Possible designs: {}", count_possible_designs(input));
}

fn second_star(input: &AOCInput) {
    println!("Possible combinations: {}", count_combinations(input));
}

fn read_input() -> Result<AOCInput> {
    let input = include_str!("../input.txt");

    let (towel_patterns, designs) = input
        .split_once("\n\n")
        .or_else(|| input.split_once("\r\n\r\n"))
        .ok_or(anyhow!(
            "Didn't find the expected two sections in AOC input"
        ))?;
    Ok(AOCInput {
        towel_patterns: towel_patterns.split(", ").map(str::to_string).collect(),
        designs: designs.lines().map(str::to_string).collect(),
    })
}

fn count_possible_designs(input: &AOCInput) -> usize {
    input
        .designs
        .par_iter()
        .filter(|d| is_possible_design(d, &input.towel_patterns))
        .count()
}

fn is_possible_design(design: &str, patterns: &[String]) -> bool {
    // Suffix lengths we already know can't be built, so we never walk them twice
    let mut dead_ends = HashSet::new();
    can_build(design, patterns, &mut dead_ends)
}

/// Tries every pattern that fits at the front of `rest` and recurses on whatever
/// follows it. An empty `rest` means the whole design was consumed.
fn can_build(rest: &str, patterns: &[String], dead_ends: &mut HashSet<usize>) -> bool {
    if rest.is_empty() {
        return true;
    }
    if dead_ends.contains(&rest.len()) {
        return false;
    }

    for pattern in patterns {
        if let Some(tail) = rest.strip_prefix(pattern.as_str())
            && can_build(tail, patterns, dead_ends)
        {
            return true;
        }
    }

    dead_ends.insert(rest.len());
    false
}

fn count_combinations(input: &AOCInput) -> u64 {
    input
        .designs
        .par_iter()
        .map(|d| count_arrangements(d, &input.towel_patterns))
        .sum()
}

fn count_arrangements(design: &str, patterns: &[String]) -> u64 {
    // Same idea as the dead ends above, but now every suffix length remembers
    // how many ways it can be built instead of just whether it is possible
    let mut known = HashMap::new();
    build_count(design, patterns, &mut known)
}

/// Every arrangement of `rest` starts with exactly one pattern, so the total is
/// the sum over all patterns that fit at the front of the arrangements of their tail.
fn build_count(rest: &str, patterns: &[String], known: &mut HashMap<usize, u64>) -> u64 {
    if rest.is_empty() {
        return 1;
    }
    if let Some(&count) = known.get(&rest.len()) {
        return count;
    }

    let mut count = 0;
    for pattern in patterns {
        if let Some(tail) = rest.strip_prefix(pattern.as_str()) {
            count += build_count(tail, patterns, known);
        }
    }

    known.insert(rest.len(), count);
    count
}
