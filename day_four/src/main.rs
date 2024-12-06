use std::collections::HashSet;

fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("../input.txt");

    let input = input
        .lines()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    part_one(&input);
    part_two(&input);
    println!("Executing took: {:?}", start_time.elapsed());
}

fn part_one(input: &Vec<Vec<char>>) {
    let possible_headings = [
        Heading { x: 0, y: -1 },
        Heading { x: 1, y: -1 },
        Heading { x: 1, y: 0 },
        Heading { x: 1, y: 1 },
        Heading { x: 0, y: 1 },
        Heading { x: -1, y: 1 },
        Heading { x: -1, y: 0 },
        Heading { x: -1, y: -1 },
    ];

    let mut total_xmas_count = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let total_xmas_count_for_index: usize =
                possible_headings.iter().fold(0, |acc, heading| {
                    let mut input_character_iterator = TwoDimensionalIterator {
                        current_index: Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        heading,
                        input,
                    };

                    static XMAS: &str = "XMAS";

                    let failed_match = XMAS.chars().any(|character| {
                        input_character_iterator
                            .next()
                            .map(|i| i != character)
                            .unwrap_or(true)
                    });

                    if failed_match {
                        acc
                    } else {
                        acc + 1
                    }
                });

            total_xmas_count += total_xmas_count_for_index;
        }
    }

    println!("Total XMAS count: {}", total_xmas_count);
}

fn part_two(input: &Vec<Vec<char>>) {
    let possible_headings = [
        Heading { x: 1, y: -1 },
        Heading { x: 1, y: 1 },
        Heading { x: -1, y: 1 },
        Heading { x: -1, y: -1 },
    ];

    let mut found = HashSet::new();
    let mut found_multiple_times = vec![];

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            possible_headings.iter().for_each(|heading| {
                let mut character_iterator = TwoDimensionalIterator {
                    current_index: Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    heading,
                    input,
                };

                static MAS: &str = "MAS";

                let failed_match = MAS.chars().any(|character| {
                    character_iterator
                        .next()
                        .map(|i| i != character)
                        .unwrap_or(true)
                });

                if !failed_match {
                    // If we've found a word, we are actually interested in the second character ('A'),
                    // because that is the middle of the two 'MAS' crossing each other.
                    let second_character_index = Point {
                        x: x as i32 + heading.x,
                        y: y as i32 + heading.y,
                    };

                    let first_insert = found.insert(second_character_index.to_owned());

                    if !first_insert {
                        found_multiple_times.push(second_character_index);
                    }
                }
            });
        }
    }

    println!("Total X-MAS count: {:?}", found_multiple_times.len());
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Heading {
    x: i32,
    y: i32,
}

struct TwoDimensionalIterator<'a> {
    current_index: Point,
    heading: &'a Heading,
    input: &'a Vec<Vec<char>>,
}

impl<'a> Iterator for TwoDimensionalIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index.y < 0
            || self.current_index.x < 0
            || self.current_index.y >= self.input.len() as i32
            || self.current_index.x >= self.input[self.current_index.y as usize].len() as i32
        {
            return None;
        }

        let item = &self.input[self.current_index.y as usize][self.current_index.x as usize];

        self.current_index = Point {
            x: self.current_index.x + self.heading.x,
            y: self.current_index.y + self.heading.y,
        };

        Some(*item)
    }
}
